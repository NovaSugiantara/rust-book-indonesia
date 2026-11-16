//! Domain inti Kasir Warung untuk Capstone.
//!
//! Modul ini tidak bergantung pada axum, tokio, atau sqlx. Aturan status,
//! perhitungan, metode pembayaran, dan rekap harian tinggal di sini agar bisa
//! diuji tanpa server atau database.

use std::collections::HashMap;
use std::fmt;

/// Status pesanan yang dipakai API dan database.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusPesanan {
    Baru,
    Dibayar,
    Diproses,
    Selesai,
    Dibatalkan,
}

impl StatusPesanan {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Baru => "baru",
            Self::Dibayar => "dibayar",
            Self::Diproses => "diproses",
            Self::Selesai => "selesai",
            Self::Dibatalkan => "dibatalkan",
        }
    }

    pub fn parse(value: &str) -> Result<Self, KasirError> {
        match value.trim().to_lowercase().as_str() {
            "baru" => Ok(Self::Baru),
            "dibayar" => Ok(Self::Dibayar),
            "diproses" => Ok(Self::Diproses),
            "selesai" => Ok(Self::Selesai),
            "dibatalkan" => Ok(Self::Dibatalkan),
            other => Err(KasirError::StatusTidakValid(format!(
                "status tidak dikenal: {other}"
            ))),
        }
    }

    /// Transisi yang boleh dilakukan lewat endpoint ubah status.
    ///
    /// Perubahan Baru ke Dibayar tidak ada di sini karena harus lewat
    /// endpoint pembayaran yang mencatat payment.
    pub fn can_transition_to(self, target: Self) -> bool {
        matches!(
            (self, target),
            (Self::Baru, Self::Dibatalkan)
                | (Self::Dibayar, Self::Diproses)
                | (Self::Diproses, Self::Selesai)
        )
    }

    pub fn bisa_dibayar(self) -> bool {
        self == Self::Baru
    }
}

/// Error domain Kasir Warung.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KasirError {
    InputTidakValid(String),
    ItemTidakDitemukan(String),
    StatusTidakValid(String),
    PembayaranGagal(String),
    Internal(String),
}

impl fmt::Display for KasirError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InputTidakValid(pesan) => write!(f, "input tidak valid: {pesan}"),
            Self::ItemTidakDitemukan(kode) => write!(f, "item tidak ditemukan: {kode}"),
            Self::StatusTidakValid(pesan) => write!(f, "status tidak valid: {pesan}"),
            Self::PembayaranGagal(pesan) => write!(f, "pembayaran gagal: {pesan}"),
            Self::Internal(pesan) => write!(f, "kesalahan internal: {pesan}"),
        }
    }
}

impl std::error::Error for KasirError {}

/// Satu baris item dalam pesanan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemPesanan {
    pub kode: String,
    pub jumlah: u32,
    pub harga: u32,
}

impl ItemPesanan {
    pub fn new(kode: impl Into<String>, jumlah: u32, harga: u32) -> Result<Self, KasirError> {
        if jumlah == 0 {
            return Err(KasirError::InputTidakValid(
                "jumlah item harus lebih dari nol".to_string(),
            ));
        }

        Ok(Self {
            kode: kode.into(),
            jumlah,
            harga,
        })
    }

    pub fn subtotal(&self) -> Result<u32, KasirError> {
        self.harga.checked_mul(self.jumlah).ok_or_else(|| {
            KasirError::InputTidakValid("subtotal item melebihi batas u32".to_string())
        })
    }
}

/// Transaksi yang sedang disusun atau sudah tersimpan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transaksi {
    pub pelanggan: String,
    pub items: Vec<ItemPesanan>,
    pub status: StatusPesanan,
}

impl Transaksi {
    pub fn baru(pelanggan: impl Into<String>, items: Vec<ItemPesanan>) -> Result<Self, KasirError> {
        let pelanggan = pelanggan.into();

        if pelanggan.trim().is_empty() {
            return Err(KasirError::InputTidakValid(
                "nama pelanggan tidak boleh kosong".to_string(),
            ));
        }

        if items.is_empty() {
            return Err(KasirError::InputTidakValid(
                "pesanan harus memiliki minimal satu item".to_string(),
            ));
        }

        Ok(Self {
            pelanggan,
            items,
            status: StatusPesanan::Baru,
        })
    }

    pub fn total(&self) -> Result<u32, KasirError> {
        self.items.iter().try_fold(0u32, |total, item| {
            total
                .checked_add(item.subtotal()?)
                .ok_or_else(|| KasirError::InputTidakValid("total melebihi batas u32".to_string()))
        })
    }

    pub fn bisa_dibayar(&self) -> bool {
        self.status.bisa_dibayar()
    }
}

/// Bukti pembayaran dari metode pembayaran.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaymentReceipt {
    pub metode: String,
    pub total: u32,
}

/// Kontrak metode pembayaran. Implementasi bisa dipilih saat runtime.
pub trait PaymentMethod: Send + Sync {
    fn kode(&self) -> &'static str;
    fn bayar(&self, total: u32) -> Result<PaymentReceipt, KasirError>;
}

pub struct Tunai;
pub struct Qris;

impl PaymentMethod for Tunai {
    fn kode(&self) -> &'static str {
        "tunai"
    }

    fn bayar(&self, total: u32) -> Result<PaymentReceipt, KasirError> {
        if total == 0 {
            return Err(KasirError::PembayaranGagal(
                "total pembayaran tidak boleh nol".to_string(),
            ));
        }

        Ok(PaymentReceipt {
            metode: self.kode().to_string(),
            total,
        })
    }
}

impl PaymentMethod for Qris {
    fn kode(&self) -> &'static str {
        "qris"
    }

    fn bayar(&self, total: u32) -> Result<PaymentReceipt, KasirError> {
        if total == 0 {
            return Err(KasirError::PembayaranGagal(
                "total pembayaran tidak boleh nol".to_string(),
            ));
        }

        Ok(PaymentReceipt {
            metode: self.kode().to_string(),
            total,
        })
    }
}

/// Memilih implementasi PaymentMethod dari kode yang dikirim client.
pub fn metode_pembayaran(kode: &str) -> Option<Box<dyn PaymentMethod>> {
    match kode.trim().to_lowercase().as_str() {
        "tunai" => Some(Box::new(Tunai)),
        "qris" => Some(Box::new(Qris)),
        _ => None,
    }
}

/// Rekap total per metode dengan iterator dan HashMap.
pub fn rekap_per_metode<I>(rows: I) -> HashMap<String, u32>
where
    I: IntoIterator<Item = (String, u32)>,
{
    rows.into_iter()
        .fold(HashMap::new(), |mut rekap, (metode, total)| {
            *rekap.entry(metode).or_insert(0) += total;
            rekap
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn total_transaksi_menjumlahkan_item() {
        let transaksi = Transaksi::baru(
            "Bu Sri",
            vec![
                ItemPesanan::new("KOPI", 2, 12_000).unwrap(),
                ItemPesanan::new("ROTI", 1, 8_000).unwrap(),
            ],
        )
        .unwrap();

        assert_eq!(transaksi.total().unwrap(), 32_000);
    }

    #[test]
    fn transisi_status_mengikuti_aturan() {
        assert!(StatusPesanan::Baru.can_transition_to(StatusPesanan::Dibatalkan));
        assert!(StatusPesanan::Dibayar.can_transition_to(StatusPesanan::Diproses));
        assert!(StatusPesanan::Diproses.can_transition_to(StatusPesanan::Selesai));
        assert!(!StatusPesanan::Baru.can_transition_to(StatusPesanan::Selesai));
        assert!(!StatusPesanan::Selesai.can_transition_to(StatusPesanan::Dibatalkan));
    }

    #[test]
    fn pembayaran_tunai_dan_qris_menghasilkan_receipt() {
        let tunai = Tunai;
        let qris = Qris;

        assert_eq!(tunai.bayar(10_000).unwrap().metode, "tunai");
        assert_eq!(qris.bayar(10_000).unwrap().metode, "qris");
    }

    #[test]
    fn rekap_per_metode_menjumlahkan() {
        let rekap = rekap_per_metode(vec![
            ("tunai".to_string(), 10_000),
            ("qris".to_string(), 20_000),
            ("tunai".to_string(), 5_000),
        ]);

        assert_eq!(rekap["tunai"], 15_000);
        assert_eq!(rekap["qris"], 20_000);
    }
}
