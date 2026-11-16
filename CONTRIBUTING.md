# Panduan menulis bab

Dokumen ini dipakai oleh penulis bab berikutnya. Isinya menjelaskan struktur file, komponen yang tersedia, aturan gaya, dan keadaan studi kasus Kasir Warung. Ikuti bagian yang relevan dengan babmu, lalu jalankan checklist di bagian akhir sebelum menganggap bab selesai.

## Prasyarat

```console
$ npm install
$ npm run dev
```

`npm run dev` menjalankan server pengembangan. `npm run build` harus tetap selesai dengan kode keluar 0 sebelum kamu mengirim pekerjaan.

## Struktur file

```text
src/
├── assets/logo.svg
├── components/
│   ├── ChapterCompletion.astro
│   ├── Footer.astro
│   ├── Progress.astro
│   ├── Quiz.astro
│   ├── RustPlayground.astro
│   ├── StudyCase.astro
│   └── StudyCaseStep.astro
├── content/docs/
│   ├── index.mdx
│   ├── bab-1/
│   │   ├── index.mdx
│   │   ├── instalasi.mdx
│   │   ├── hello-world.mdx
│   │   ├── hello-cargo.mdx
│   │   └── studi-kasus.mdx
│   ├── bab-2/
│   │   ├── index.mdx
│   │   ├── persiapan.mdx
│   │   ├── input.mdx
│   │   ├── angka-rahasia.mdx
│   │   ├── membandingkan.mdx
│   │   ├── loop.mdx
│   │   └── studi-kasus.mdx
│   ├── bab-3/
│   │   ├── index.mdx
│   │   ├── variabel-mutability.mdx
│   │   ├── tipe-data.mdx
│   │   ├── fungsi.mdx
│   │   ├── komentar.mdx
│   │   ├── control-flow.mdx
│   │   └── studi-kasus.mdx
│   └── bab-4.mdx ... bab-21.mdx
├── data/chapters.ts
├── scripts/progress.ts
└── styles/custom.css
```

Setiap bab punya satu halaman ringkasan. Bab yang sudah ditulis boleh memecah materinya menjadi beberapa halaman di folder `bab-N/`. Bab yang belum ditulis tetap punya satu file `bab-N.mdx` berisi halaman "Segera hadir".

Sidebar dan ringkasan progres dibangun dari `src/data/chapters.ts`. Kalau kamu menambah halaman baru, tambahkan juga entri di sana:

```ts
{
  number: 2,
  title: 'Membuat game tebak angka',
  slug: 'bab-2',
  pages: [
    { label: 'Ringkasan bab', slug: 'bab-2' },
    { label: '2.1 ...', slug: 'bab-2/...' },
  ],
}
```

Jangan mengubah nomor atau urutan 21 bab. Slug memakai huruf kecil dan tanda hubung.

### Mengganti halaman Segera hadir

Saat sebuah bab mulai ditulis, ganti isi `bab-N.mdx` menjadi halaman ringkasan yang sebenarnya, atau pindahkan menjadi `bab-N/index.mdx` lalu tambahkan subhalaman di folder yang sama. Jangan menyisakan `bab-N.mdx` dan `bab-N/index.mdx` sekaligus karena keduanya menghasilkan path yang sama. Setelah itu, perbarui `pages` di `src/data/chapters.ts` supaya sidebar dan progres mengenali halaman baru.

## Template halaman

Halaman di dalam folder bab memakai import seperti ini:

```mdx
---
title: '2.1 Contoh halaman'
description: 'Satu kalimat tentang isi halaman.'
---

import ChapterCompletion from '../../../components/ChapterCompletion.astro';
import Quiz from '../../../components/Quiz.astro';
import RustPlayground from '../../../components/RustPlayground.astro';
import StudyCase from '../../../components/StudyCase.astro';
import StudyCaseStep from '../../../components/StudyCaseStep.astro';

Isi halaman dimulai dengan paragraf biasa. Jangan menulis H1 lagi karena Starlight sudah menampilkan `title` sebagai H1.

## Subjudul pertama

...
```

Halaman yang berada langsung di `src/content/docs/` memakai `../../components/` untuk import.

## Komponen

### RustPlayground

Blok kode yang bisa diubah dan dijalankan lewat API playground Rust publik.

```mdx
<RustPlayground
  code={`fn main() {
    println!("Halo");
}`}
  filename="src/main.rs"
/>
```

Props:

| Prop | Tipe | Default | Keterangan |
|---|---|---|---|
| `code` | string | wajib | Kode Rust yang ditampilkan. |
| `editable` | boolean | `true` | Kalau `false`, kode ditampilkan sebagai blok statis dan hanya tombol Jalankan yang tersedia. |
| `expectError` | boolean | `false` | Untuk kode yang sengaja gagal dikompilasi. Komponen menampilkan pesan bahwa error memang diharapkan. |
| `filename` | string | `main.rs` | Label file di bilah atas. |
| `edition` | `'2021'` atau `'2024'` | `'2024'` | Edition yang dikirim ke playground. |
| `label` | string | teks umum | Nama aksesibel untuk region playground. Isi dengan tujuan contoh. |

Perilaku:

- Tombol Jalankan mengirim POST ke `https://play.rust-lang.org/execute` dengan `channel: "stable"`, `mode: "debug"`, `crateType: "bin"`, dan edition dari props.
- stdout dan stderr ditampilkan sebagai teks, bukan HTML.
- Kalau jaringan gagal atau timeout 30 detik, muncul pesan berbahasa Indonesia.
- Tautan Buka di Playground membawa kode terbaru setelah editor diubah.
- Gunakan `expectError` untuk contoh yang memang salah. Jangan memakai `expectError` untuk kode yang seharusnya jalan.

Semua kode yang ditampilkan harus sudah dicoba. Kalau tidak yakin outputnya, jalankan lewat playground sebelum menulis penjelasan.

#### Contoh yang membutuhkan input

Playground tidak menyediakan stdin interaktif. Kode yang memanggil `read_line` akan langsung mendapat akhir input, bukan ketikan pengguna. Untuk contoh seperti itu, tampilkan blok kode biasa tanpa tombol Jalankan, atau buat versi playground yang memakai nilai tetap. Jangan memasang RustPlayground pada kode yang menunggu input, karena pembaca akan mengira contohnya rusak.

### Quiz

Kuis pilihan ganda dengan umpan balik langsung. Satu halaman sebaiknya memakai satu komponen Quiz berisi semua pertanyaan, karena skor disimpan per halaman.

```mdx
<Quiz
  questions={[
    {
      question: 'Apa output dari kode ini?',
      code: 'fn main() {\n    println!("Halo");\n}',
      options: ['Halo', 'Halo\\n', 'Tidak ada output'],
      answer: 0,
      explanation: 'println! mencetak teks lalu menambahkan baris baru.',
      optionNotes: [
        'Benar. Teks dicetak tanpa tanda kutip.',
        'Salah. Baris baru tidak terlihat sebagai karakter \\n di layar.',
        'Salah. Program mencetak satu baris.',
      ],
    },
  ]}
/>
```

Aturan data:

- `answer` memakai indeks mulai dari 0.
- Panjang `optionNotes` harus sama dengan panjang `options`. Setiap opsi butuh alasan, termasuk opsi yang salah.
- `code` opsional. Pakai untuk soal tebak output.
- `explanation` opsional, dipakai sebagai penjelasan umum dan cadangan.
- Setiap halaman materi minimal punya 3 pertanyaan. Soal hafalan sintaks tanpa konteks sebaiknya diganti soal output atau alasan.
- Jangan menaruh dua komponen Quiz di satu halaman. Kalau butuh banyak soal, gabungkan dalam satu array.

Skor disimpan bersama progres halaman di `localStorage`. Kalau penyimpanan diblokir, kuis tetap berjalan sampai halaman ditutup.

### StudyCase dan StudyCaseStep

Blok studi kasus dengan langkah bernomor. Setiap langkah punya tujuan, isi, hint yang bisa dibuka, solusi yang bisa dibuka, dan boleh berisi playground.

```mdx
<StudyCase title="Kasir Warung: langkah berikutnya" intro="Ringkasan singkat misi.">
  <StudyCaseStep title="Baca input" goal="Menerima nama pelanggan dari terminal.">
    <Fragment slot="hint">
      Gunakan `std::io::stdin()` dan `read_line`.
    </Fragment>

    <Fragment slot="solution">
      ```rust
      // solusi lengkap di sini
      ```
    </Fragment>

    Isi langkah, penjelasan, dan playground kalau perlu.
  </StudyCaseStep>
</StudyCase>
```

Slot `hint` dan `solution` opsional. Kalau tidak diisi, tombolnya tidak dirender. Isi slot bisa Markdown, termasuk blok kode.

### ChapterCompletion

Tombol "Tandai selesai" untuk satu halaman. Letakkan di bagian paling bawah setiap halaman materi, setelah kuis.

```mdx
<ChapterCompletion />
```

Komponen menyimpan status di `localStorage` dan memancarkan event `rustbook:progress` supaya komponen lain di halaman yang sama ikut memperbarui tampilan.

### Progress

Ringkasan progres untuk halaman depan. Komponen ini membaca daftar bab dari `src/data/chapters.ts` dan menampilkan jumlah halaman selesai, skor kuis, dan progres per bab.

```mdx
<Progress />
```

Pakai hanya di `src/content/docs/index.mdx`. Komponen ini memakai id heading `progress-title`, jadi menaruhnya dua kali di satu halaman akan menghasilkan id ganda.

### Callout

Gunakan aside bawaan Starlight. Jangan membuat komponen baru untuk callout.

```mdx
:::caution[Hati-hati]
Peringatan tentang hal yang bisa merusak data atau membuat kode gagal.
:::

:::tip[Tips]
Cara yang lebih cepat atau lebih nyaman.
:::

:::note[Kenapa begini?]
Alasan di balik keputusan desain Rust atau urutan penjelasan.
:::
```

Label boleh diganti sesuai konteks. Jangan memakai emoji di judul.

## Aturan menulis

Aturan ini keras. Konten yang melanggar akan ditolak.

1. Bahasa Indonesia, menyapa pembaca dengan "kamu". Bukan "Anda", bukan "kalian".
2. Tidak ada em dash atau en dash di prosa. Pakai titik, koma, atau tanda kurung. Perintah terminal dengan dua tanda hubung seperti `--release` tetap boleh.
3. Tidak ada emoji, termasuk di heading dan callout.
4. Heading memakai sentence case. Contoh: "Mengompilasi dan menjalankan", bukan "Mengompilasi Dan Menjalankan".
5. Hindari frasa pengisi seperti "mari kita selami", "sangat penting untuk dicatat", "di era modern", "pada akhirnya nanti", dan "dengan demikian maka".
6. Jangan membuat bullet list yang setiap itemnya berupa header tebal. Pakai paragraf kalau gagasannya perlu dijelaskan. List dipakai untuk urutan perintah atau kumpulan item pendek.
7. Vary sentence length. Kalimat pendek untuk penekanan, kalimat panjang untuk alasan. Jangan menulis semua kalimat dengan pola yang sama.
8. Istilah teknis Rust tetap dalam bahasa Inggris saat istilah itu lazim dipakai developer Indonesia: ownership, borrow, trait, crate, struct, enum, match, closure, iterator. Jelaskan singkat saat pertama muncul. Contoh: "Ownership adalah aturan tentang siapa yang bertanggung jawab membersihkan sebuah nilai."
9. Jangan menerjemahkan nama tipe, fungsi, atau perintah. `String`, `Vec`, `cargo build`, dan `println!` tetap apa adanya.
10. Jangan mengarang fakta, output, atau perilaku Rust. Kalau ragu, buka buku asli atau dokumentasi resmi. Tautkan sumbernya di akhir halaman kalau topiknya mudah berubah.
11. Jangan mengarang statistik, testimoni, jumlah pembaca, atau klaim manfaat yang tidak bisa dibuktikan.
12. Kode harus bisa dijalankan kecuali memang ditandai `expectError`. Jangan menampilkan pseudo-code sebagai Rust yang valid.
13. Sebut nama file di atas blok kode kalau konteksnya penting. Gunakan prop `filename` pada RustPlayground.
14. Tautan internal memakai path relatif, misalnya `[Instalasi](instalasi/)` dari halaman bab atau `[Bab 1](../bab-1/)` dari halaman lain. Jangan memakai `/bab-1/`, karena base GitHub Pages tidak ikut ditambahkan ke link absolut.
15. Materi mengikuti urutan buku. Jangan memakai konsep dari bab yang jauh lebih depan tanpa menjelaskannya singkat.

## Studi kasus Kasir Warung

Studi kasus ini menyatukan materi bab menjadi satu proyek yang tumbuh. Bab 1 sampai Bab 3 punya rencana yang sudah disepakati. Bab berikutnya boleh melanjutkan selama tidak merusak kontrak output.

### Keadaan setelah Bab 1

Bab 1 membuat proyek Cargo dan mencetak header struk. Proyek belum punya dependency, variabel, input, atau perhitungan.

```rust
fn main() {
    println!("========================================");
    println!("           KASIR WARUNG BU SRI");
    println!("        Jl. Melati No. 12, Bandung");
    println!("========================================");
    println!();
}
```

Kontrak yang perlu dijaga:

- Lebar garis pemisah 40 karakter.
- Header berisi nama warung, alamat, garis pemisah, lalu satu baris kosong.
- Program dijalankan dengan `cargo run` dari folder `kasir_warung`.
- Belum ada input. Output harus sama setiap kali dijalankan.

### Keadaan setelah Bab 2

Bab 2 menambahkan validasi jumlah barang. Program masih mencetak header yang sama, lalu meminta jumlah barang sampai mendapat angka 1 sampai 3. Input non-angka ditolak lewat `parse` yang gagal dan `continue`. Angka di luar rentang ditolak lewat `match` pada rentang `1..=3`.

```rust
use std::io;

fn main() {
    println!("========================================");
    println!("           KASIR WARUNG BU SRI");
    println!("        Jl. Melati No. 12, Bandung");
    println!("========================================");

    let mut jumlah_barang = 0;

    loop {
        println!("Jumlah barang (1-3):");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Gagal membaca input");

        let jumlah: u32 = match input.trim().parse() {
            Ok(angka) => angka,
            Err(_) => {
                println!("Masukkan angka, bukan teks. Coba lagi.");
                continue;
            }
        };

        match jumlah {
            1..=3 => {
                jumlah_barang = jumlah;
                break;
            }
            _ => println!("Jumlah harus 1 sampai 3. Coba lagi."),
        }
    }

    println!();
    println!("Jumlah barang: {jumlah_barang}");
    println!("========================================");
}
```

Kontrak setelah Bab 2:

- Header tetap 40 karakter dan tidak berubah dari Bab 1.
- Jumlah barang yang diterima selalu 1 sampai 3.
- Input salah tidak membuat program panic. Program meminta input lagi.
- Output akhir menampilkan `Jumlah barang: N`.
- Belum ada harga, item, subtotal, atau total.
- Batas 3 dipilih karena Bab 3 memakai array berukuran tetap. Bab 8 akan menggantinya dengan `Vec`.

### Keadaan setelah Bab 3

Bab 3 menambahkan perhitungan ke Kasir Warung. Validasi jumlah barang dari Bab 2 tetap dipakai. Setelah jumlah diterima, program membaca array harga dan jumlah contoh, menjumlahkan subtotal dengan `for`, menghitung diskon dengan `if`, lalu mencetak total.

```rust
use std::io;

fn hitung_subtotal(harga: u32, jumlah: u32) -> u32 {
    harga * jumlah
}

fn hitung_diskon(subtotal: u32) -> u32 {
    if subtotal >= 40_000 {
        subtotal * 10 / 100
    } else {
        0
    }
}

fn main() {
    println!("========================================");
    println!("           KASIR WARUNG BU SRI");
    println!("        Jl. Melati No. 12, Bandung");
    println!("========================================");

    let mut jumlah_barang: usize = 0;

    loop {
        println!("Jumlah barang (1-3):");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Gagal membaca input");

        let jumlah: usize = match input.trim().parse() {
            Ok(angka) => angka,
            Err(_) => {
                println!("Masukkan angka, bukan teks. Coba lagi.");
                continue;
            }
        };

        match jumlah {
            1..=3 => {
                jumlah_barang = jumlah;
                break;
            }
            _ => println!("Jumlah harus 1 sampai 3. Coba lagi."),
        }
    }

    let daftar_harga = [12_000, 8_000, 15_000];
    let daftar_jumlah = [1, 2, 1];

    let mut subtotal = 0;

    for i in 0..jumlah_barang {
        let baris = hitung_subtotal(daftar_harga[i], daftar_jumlah[i]);
        subtotal += baris;
        println!("Item {}: Rp{baris}", i + 1);
    }

    let diskon = hitung_diskon(subtotal);
    let total = subtotal - diskon;

    println!("----------------------------------------");
    println!("Subtotal: Rp{subtotal}");
    println!("Diskon:   Rp{diskon}");
    println!("Total:    Rp{total}");
    println!("========================================");
}
```

Kontrak setelah Bab 3:

- Header tetap 40 karakter.
- Jumlah barang tetap divalidasi 1 sampai 3.
- Harga disimpan sebagai integer rupiah, bukan floating point.
- Subtotal dihitung dari array `daftar_harga` dan `daftar_jumlah`.
- Diskon 10 persen diberikan saat subtotal minimal 40.000.
- Output mencetak baris item, subtotal, diskon, dan total.
- Belum memakai struct, enum, atau `Vec`.
- Batas tiga item berasal dari panjang array tetap. Bab 8 akan menggantinya dengan `Vec`.
- Format mata uang masih sederhana, yaitu `Rp` diikuti angka tanpa pemisah ribuan.
- Tipe `jumlah_barang` berubah dari `u32` di Bab 2 menjadi `usize` di Bab 3 karena dipakai sebagai indeks array.

### Aturan untuk bab berikutnya

- Jangan mengubah header 40 karakter tanpa memperbarui bab sebelumnya.
- Kalau format baris item ditambahkan, tulis formatnya di panduan ini dan pakai konsisten.
- Jangan memakai fitur dari bab yang belum diajarkan tanpa penjelasan singkat. Kalau terpaksa, jelaskan alasannya di callout.
- Setiap bab yang menyentuh Kasir Warung harus meninggalkan kode yang bisa dijalankan dan output contoh yang benar.

## Progres dan localStorage

Kunci penyimpanan: `rustbook:progress:v1`.

Bentuk datanya:

```json
{
  "/bab-1/hello-world/": {
    "completed": true,
    "completedAt": "2026-01-01T00:00:00.000Z",
    "quiz": {
      "correct": 3,
      "total": 4,
      "answers": { "0": 0, "1": 2, "2": 1 }
    }
  }
}
```

Path halaman selalu diakhiri garis miring. Semua akses `localStorage` dibungkus `try/catch` lewat `src/scripts/progress.ts`. Jangan memanggil `localStorage` langsung dari komponen baru tanpa penanganan yang sama.

Event `rustbook:progress` dipancarkan setiap kali data berubah. Komponen yang menampilkan progres boleh mendengarkan event itu. Jangan menghapus kunci ini saat pengguna membuka halaman baru.

## Internasionalisasi

Bahasa Indonesia adalah locale root. Halaman berbahasa Indonesia berada langsung di `src/content/docs/`. Bahasa Inggris dikonfigurasi sebagai locale kedua, tetapi belum punya konten terjemahan. Starlight memakai konten default sebagai fallback untuk `/en/`.

Kalau nanti ada terjemahan Inggris, buat file dengan path yang sama di `src/content/docs/en/`, misalnya `src/content/docs/en/bab-1/instalasi.mdx`. Jangan menyalin konten Indonesia ke folder `en` tanpa menerjemahkan.

## Checklist per bab

Sebelum menyatakan bab selesai:

- [ ] Frontmatter `title` dan `description` berbahasa Indonesia dan tidak kosong.
- [ ] Halaman baru sudah ditambahkan ke `src/data/chapters.ts`.
- [ ] Setiap halaman materi punya minimal 3 pertanyaan kuis dengan alasan untuk setiap opsi.
- [ ] Semua kode Rust sudah dicoba di playground. Kode yang sengaja gagal memakai `expectError`.
- [ ] Setiap halaman punya `<ChapterCompletion />` di bagian paling bawah.
- [ ] Ada minimal satu callout yang relevan, bukan callout tempelan.
- [ ] Studi kasus dilanjutkan atau dijelaskan kenapa bab ini belum menyentuhnya.
- [ ] Tidak ada em dash atau en dash di `src/content`.
- [ ] Tidak ada emoji di heading atau callout.
- [ ] Tidak ada istilah teknis yang dipakai tanpa penjelasan saat pertama muncul.
- [ ] Tautan internal dan eksternal bisa dibuka.
- [ ] `npm run build` selesai dengan kode keluar 0.
- [ ] Halaman diperiksa di tema terang, tema gelap, dan lebar ponsel.
- [ ] Tombol dan pilihan kuis bisa dipakai dengan keyboard.
- [ ] README bagian "Isi saat ini" diperbarui kalau status bab berubah.

## Perintah pemeriksaan

```console
$ npm run build
$ grep -rn "—\|–" src/content
$ grep -rn "😀\|🚀" src/content
```

Perintah pertama harus berakhir dengan "Complete!" dan kode keluar 0. Dua perintah berikut harus tidak mencetak apa pun.

Untuk menguji playground tanpa browser, kirim payload yang sama dengan komponen:

```console
$ curl -sS -X POST https://play.rust-lang.org/execute \
  -H 'Content-Type: application/json' \
  -d '{"channel":"stable","mode":"debug","edition":"2024","crateType":"bin","tests":false,"backtrace":false,"code":"fn main() { println!(\"Halo\"); }"}'
```

Respons sukses berisi `success: true` dan `stdout` berisi `Halo`.

## Atribusi

Footer dan README sudah memuat atribusi ke The Rust Programming Language oleh Steve Klabnik, Carol Nichols, dan Chris Krycho, dengan kontribusi dari komunitas Rust, lisensi MIT atau Apache-2.0. Jangan menghapus atribusi itu saat menambah halaman.
