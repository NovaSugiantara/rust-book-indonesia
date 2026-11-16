# Panduan menulis bab

Dokumen ini dipakai oleh penulis bab berikutnya. Isinya menjelaskan pembaca sasaran, peta kurikulum, struktur file, komponen yang tersedia, aturan gaya, dan keadaan studi kasus Kasir Warung. Ikuti bagian yang relevan dengan babmu, lalu jalankan checklist di bagian akhir sebelum menganggap bab selesai.

## Pembaca dan tujuan

Pembaca utama situs ini adalah fullstack atau backend web developer yang sudah nyaman menulis kode di JavaScript/TypeScript, PHP, Go, atau Python. Mereka sudah memahami HTTP, database relasional, dan REST, tetapi belum tentu pernah memakai Rust.

Tujuannya bukan sekadar bisa membuat CRUD. Pembaca harus keluar dengan pemahaman tentang:

- aturan ownership dan borrow, serta kapan memilih `&str`, `String`, atau clone;
- struct, enum, pattern matching, dan cara memodelkan data transaksi;
- error handling dengan `Result`, `?`, dan response HTTP;
- trait, generic, lifetime, dan trait object untuk abstraksi pembayaran;
- test otomatis untuk logika dan endpoint;
- iterator dan closure untuk laporan;
- smart pointer, `Arc`, `Mutex`, `RwLock`, dan channel untuk state bersama;
- thread dan async Rust, termasuk perbedaan keduanya;
- web framework Rust dan cara membangun layanan backend yang bisa dijalankan.

Situs ini tidak mengajarkan dasar pemrograman dari nol. Pembaca diasumsikan sudah bisa memakai terminal, Git, package manager, dan membaca pesan error. Materi Rust dimulai dari instalasi di Bab 1.

The Rust Book tetap menjadi tulang punggung urutan materi. Konten situs adalah adaptasi, bukan terjemahan. Tambahan berupa studi kasus, capstone, dan proyek bonus harus selalu jelas statusnya dan tidak boleh mengarang fakta atau API.

## Peta kurikulum dan studi kasus

Kasir Warung dimulai sebagai CLI di Bab 1 dan tumbuh menjadi layanan backend. Setiap bab harus memakai konsep dari bab itu. Kalau buku tidak cocok dengan tema warung, adaptasi tema secukupnya, tetapi konsep bab tetap harus muncul.

| Bab | Konsep utama dari buku | Peran di Kasir Warung |
| --- | --- | --- |
| 1 | Cargo, `rustc`, program pertama | Proyek `kasir_warung`, header struk CLI |
| 2 | `std::io`, `rand`, `match`, `loop` | Input jumlah barang 1 sampai 3, validasi dengan loop dan match |
| 3 | Variabel, tipe, fungsi, `if`, loop | `hitung_subtotal`, diskon, total dari array harga |
| 4 | Ownership, borrow, slice | Nama item `String` dimiliki transaksi; fungsi menerima `&str` atau slice, clone hanya saat perlu |
| 5 | Struct dan method | `MenuItem`, `ItemPesanan`, `Transaksi`, method subtotal dan total |
| 6 | Enum, `Option`, `match` | `StatusPesanan`, status bayar, dan diskon opsional |
| 7 | Package, crate, module | Pisahkan `menu`, `transaksi`, dan `laporan`; `lib.rs` plus `main.rs` |
| 8 | `Vec`, `String`, `HashMap` | Daftar item pesanan, menu berdasarkan kode, nama item |
| 9 | `panic!`, `Result`, `?` | Error validasi dan pembayaran sebagai nilai, bukan panic |
| 10 | Generic, trait, lifetime | `trait PaymentMethod`, fungsi generik pembayaran, lifetime pada data pinjaman |
| 11 | Test otomatis | Unit test subtotal dan diskon, integration test alur bayar, kasus error |
| 12 | Proyek I/O CLI | CLI laporan harian dari file transaksi, argumen, modul, dan test |
| 13 | Iterator dan closure | Rekap transaksi: filter status, map total, fold, kelompokkan metode bayar |
| 14 | Cargo dan crates.io | Workspace `kasir-core` dan `kasir-cli`, profil release, paket siap dibagikan |
| 15 | Smart pointer | `Rc<RefCell<MenuCache>>` untuk cache single-thread dan batasnya |
| 16 | Thread, `Arc`, `Mutex`, channel | Worker memproses transaksi, state bersama, antrean lewat channel |
| 17 | Async, `await`, future, stream | Pembayaran dan akses data async dengan Tokio, join beberapa tugas |
| 18 | OOP dan trait object | `Box<dyn PaymentMethod>` untuk registry metode pembayaran |
| 19 | Pattern dan matching | Parsing perintah atau request, destructuring enum, match guard |
| 20 | Fitur lanjutan | Makro kecil untuk validasi atau error response, trait bound lanjutan |
| 21 | Proyek akhir web server | Server TCP manual dengan thread pool, endpoint health dan transaksi |
| Capstone | axum, Tokio, SQLite, `sqlx` | REST API Kasir Warung, state bersama, error HTTP, test endpoint, release |

Aturan peta:

- Bab 4 sampai 15 sudah ditulis. Jangan menulis bab 16 sampai 21 sebelum ada tugas khusus untuk bab itu.
- Setiap bab yang menyentuh Kasir Warung harus meninggalkan kode yang bisa dijalankan dan output contoh yang benar.
- Jangan memakai fitur dari bab yang belum diajarkan tanpa penjelasan singkat. Kalau terpaksa, jelaskan alasannya di callout.
- Bab 4 dan 5 mengubah cara data transaksi disimpan. Jangan mempertahankan array harga dari Bab 3 kalau struct sudah tersedia.
- Bab 9 mengganti panic pada input pengguna dengan `Result`. Ini titik perubahan gaya error dari CLI menjadi layanan.
- Bab 16 dan 17 berbeda. Thread memakai `std::thread` dan sinkronisasi blocking; async memakai future dan executor. Jangan mencampur keduanya tanpa menjelaskan bedanya.
- Bab 21 tetap membangun server dari nol. Capstone baru memakai framework.

## Track Capstone

Capstone adalah materi tambahan di luar 21 bab The Rust Book. Statusnya terpisah dari buku dan tidak boleh disamakan dengan Bab 21.

Tujuan capstone adalah mengubah Kasir Warung menjadi layanan HTTP dengan axum dan Tokio, memakai SQLite lewat `sqlx`. Cakupan wajibnya:

- routing, handler async, dan extractor `Path`, `Json`, `State`;
- shared state dengan `Arc`;
- error aplikasi yang dikonversi menjadi response HTTP memakai `IntoResponse`;
- database async SQLite dengan `sqlx`, termasuk migrasi;
- test endpoint tanpa membuka port jaringan;
- konfigurasi lewat environment variable, graceful shutdown, `cargo build --release`, dan cara menjalankan binary.

Halaman capstone:

- `src/content/docs/capstone/index.mdx`: ringkasan, prasyarat, rencana endpoint, dan daftar versi.
- `src/content/docs/capstone/framework.mdx`: perbandingan singkat axum, actix-web, dan rocket, plus alasan memilih axum.

Halaman rincian endpoint dan database ditulis setelah Bab 4 sampai Bab 21 selesai. Selama belum ditulis, jangan menampilkan contoh kode framework yang seolah sudah jadi.

Sumber resmi wajib untuk capstone:

- [docs.rs/axum](https://docs.rs/axum/)
- [docs.rs/tokio](https://docs.rs/tokio/)
- [docs.rs/sqlx](https://docs.rs/sqlx/)
- dokumentasi resmi framework yang sedang dipakai

Per 16 November 2026, versi stabil yang dipakai dalam rencana adalah axum 0.8.9, tokio 1.54.1, dan sqlx 0.9.0. Cek ulang sebelum menulis kode. Jangan mengarang nama module, tipe, atau signature.

Alasan memilih axum ada di halaman perbandingan. Ringkasnya: axum berada di ekosistem Tokio yang sama dengan Bab 17, memakai hyper dan tower, dan memperlihatkan routing serta extractor secara eksplisit. Itu alasan kurikulum, bukan klaim bahwa axum selalu lebih cepat atau lebih baik dari framework lain.

## Proyek Bonus

Proyek Bonus adalah URL shortener dengan statistik klik di folder `proyek-bonus/` di root repo. Ini bukan bagian dari 21 bab dan bukan Capstone. Tujuannya adalah contoh jadi yang memakai seluruh konsep Rust Book dalam satu layanan kecil.

- Kode lengkap ada di `proyek-bonus/` dan tidak ikut build Astro.
- Halaman situs ada di `src/content/docs/proyek-bonus/`: pengantar plus empat langkah.
- `proyek-bonus/COVERAGE.md` adalah matriks konsep ke `file:baris`.
- `proyek-bonus/scripts/check-coverage.mjs` memeriksa semua referensi file dan baris.
- Versi crate mengikuti docs.rs dan crates.io. Saat ini axum 0.8.9, tokio 1.54.1, sqlx 0.9.0, async-trait 0.1.92, rand 0.10.4.
- Kode yang membutuhkan server atau database tidak dipasang di RustPlayground. Tampilkan sebagai blok kode biasa beserta perintah `cargo run` dan `curl`.
- Setiap halaman langkah memakai `StudyCase`, memiliki blok `Kalau di bahasa lain`, dan diakhiri kuis.

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
│   ├── bab-4/
│   │   ├── index.mdx
│   │   ├── what-is-ownership.mdx
│   │   ├── references-borrowing.mdx
│   │   ├── slices.mdx
│   │   └── studi-kasus.mdx
│   ├── bab-5/
│   │   ├── index.mdx
│   │   ├── defining-structs.mdx
│   │   ├── example-structs.mdx
│   │   ├── method-syntax.mdx
│   │   └── studi-kasus.mdx
│   ├── bab-6/
│   │   ├── index.mdx
│   │   ├── defining-enums.mdx
│   │   ├── match.mdx
│   │   ├── if-let.mdx
│   │   └── studi-kasus.mdx
│   ├── bab-7/
│   │   ├── index.mdx
│   │   ├── packages-crates.mdx
│   │   ├── modules.mdx
│   │   ├── paths.mdx
│   │   ├── use.mdx
│   │   ├── files.mdx
│   │   └── studi-kasus.mdx
│   ├── bab-8/
│   │   ├── index.mdx
│   │   ├── vectors.mdx
│   │   ├── strings.mdx
│   │   ├── hash-maps.mdx
│   │   └── studi-kasus.mdx
│   ├── bab-9/
│   │   ├── index.mdx
│   │   ├── panic.mdx
│   │   ├── result.mdx
│   │   ├── panic-or-result.mdx
│   │   └── studi-kasus.mdx
│   ├── bab-10/
│   │   ├── index.mdx
│   │   ├── generics.mdx
│   │   ├── traits.mdx
│   │   ├── lifetimes.mdx
│   │   └── studi-kasus.mdx
│   ├── bab-11/
│   │   ├── index.mdx
│   │   ├── writing-tests.mdx
│   │   ├── running-tests.mdx
│   │   ├── test-organization.mdx
│   │   └── studi-kasus.mdx
│   ├── bab-12/
│   │   ├── index.mdx
│   │   ├── accepting-arguments.mdx
│   │   ├── reading-file.mdx
│   │   ├── refactoring.mdx
│   │   ├── tdd.mdx
│   │   ├── environment-variables.mdx
│   │   ├── stderr.mdx
│   │   └── studi-kasus.mdx
│   ├── bab-13/
│   │   ├── index.mdx
│   │   ├── closures.mdx
│   │   ├── iterators.mdx
│   │   ├── improving-io.mdx
│   │   ├── performance.mdx
│   │   └── studi-kasus.mdx
│   ├── bab-14/
│   │   ├── index.mdx
│   │   ├── release-profiles.mdx
│   │   ├── publishing.mdx
│   │   ├── workspaces.mdx
│   │   ├── cargo-install.mdx
│   │   ├── custom-commands.mdx
│   │   └── studi-kasus.mdx
│   ├── bab-15/
│   │   ├── index.mdx
│   │   ├── box.mdx
│   │   ├── deref.mdx
│   │   ├── drop.mdx
│   │   ├── rc.mdx
│   │   ├── interior-mutability.mdx
│   │   ├── reference-cycles.mdx
│   │   └── studi-kasus.mdx
│   ├── capstone/
│   │   ├── index.mdx
│   │   └── framework.mdx
│   ├── proyek-bonus/
│   │   ├── index.mdx
│   │   ├── langkah-1.mdx
│   │   ├── langkah-2.mdx
│   │   ├── langkah-3.mdx
│   │   └── langkah-4.mdx
│   └── bab-16.mdx ... bab-21.mdx
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
16. Setiap bab punya satu blok `:::note[Kalau di bahasa lain]` yang singkat. Bandingkan dengan JS/TS, Go, PHP, atau Python hanya kalau perbandingannya membantu. Contoh yang sah: ownership vs garbage collector, `Result` vs `try/catch`, trait vs interface, async Rust vs Promise atau goroutine, `cargo` vs npm/composer/go mod. Jangan memaksakan perbandingan dan jangan menulis klaim performa tanpa sumber.
17. Materi capstone dan proyek bonus memakai dokumentasi resmi serta versi crate yang dicek ke docs.rs. Jangan menyalin API dari ingatan. Tautkan sumbernya di halaman terkait.

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

### Keadaan setelah Bab 4

Bab 4 menambahkan nama pelanggan sebagai `String` dan mengubah fungsi baca menjadi peminjaman. `cetak_header` menerima `&str`, sedangkan `hitung_subtotal` menerima dua slice `&[u32]`. Array harga dan jumlah tetap dimiliki `main`; tidak ada clone.

```rust
fn cetak_header(nama: &str, jumlah_barang: usize) {
    // mencetak header dan nama pelanggan
}

fn hitung_subtotal(harga: &[u32], jumlah: &[u32]) -> u32 {
    let mut subtotal = 0;
    for i in 0..harga.len() {
        subtotal += harga[i] * jumlah[i];
    }
    subtotal
}
```

Pemanggilan di `main` memakai potongan array:

```rust
let subtotal = hitung_subtotal(
    &daftar_harga[..jumlah_barang],
    &daftar_jumlah[..jumlah_barang],
);
```

Kontrak setelah Bab 4:

- Nama pelanggan dimiliki `main` sebagai `String`, lalu dipinjam sebagai `&str`.
- Data item masih berupa dua array: `daftar_harga` dan `daftar_jumlah`.
- Fungsi hanya membaca data dan tidak mengambil ownership.
- Tidak ada clone otomatis. Clone hanya boleh muncul kalau ada dua pemilik independen.
- Belum ada struct. Bab 5 akan mengganti array terpisah dengan struct.
- Batas 3 item masih berlaku.

### Keadaan setelah Bab 5

Bab 5 mengganti array terpisah dengan struct. Data item sekarang dimiliki oleh `ItemPesanan`, dan `Transaksi` memiliki pelanggan, array tiga item, serta jumlah item yang dipakai.

```rust
#[derive(Debug)]
struct ItemPesanan {
    nama: String,
    harga: u32,
    jumlah: u32,
}

impl ItemPesanan {
    fn baru(nama: &str, harga: u32, jumlah: u32) -> Self;
    fn subtotal(&self) -> u32;
}

#[derive(Debug)]
struct Transaksi {
    pelanggan: String,
    items: [ItemPesanan; 3],
    jumlah_item: usize,
}

impl Transaksi {
    fn baru(pelanggan: String, items: [ItemPesanan; 3], jumlah_item: usize) -> Self;
    fn subtotal(&self) -> u32;
    fn diskon(&self) -> u32;
    fn total(&self) -> u32;
    fn cetak(&self);
}
```

Kontrak setelah Bab 5:

- `ItemPesanan::baru` adalah associated function dan memiliki `nama` sebagai `String`.
- `Transaksi` memiliki item dan pelanggan; tidak ada array harga terpisah.
- Subtotal dihitung dengan method dan slice `self.items[..self.jumlah_item]`.
- Diskon tetap 10 persen saat subtotal minimal 40.000.
- Array masih berukuran tiga karena `Vec` baru dibahas di Bab 8.
- Belum ada status pesanan, metode pembayaran, atau `Result`.

### Keadaan setelah Bab 6

Bab 6 menambahkan status pesanan dan metode pembayaran sebagai enum. `Transaksi` sekarang memiliki status, metode bayar opsional, dan diskon opsional.

```rust
enum StatusPesanan {
    Baru,
    Diproses,
    Selesai,
    Dibatalkan,
}

enum MetodeBayar {
    Tunai,
    Qris,
    Transfer,
}

struct Transaksi {
    pelanggan: String,
    items: [ItemPesanan; 3],
    jumlah_item: usize,
    status: StatusPesanan,
    metode_bayar: Option<MetodeBayar>,
    diskon: Option<u32>,
}
```

Perilaku utama:

- `StatusPesanan::label` memakai `match` untuk semua variant.
- `Transaksi::bayar` mengisi `metode_bayar` dan mengubah status menjadi `Selesai`.
- `hitung_diskon` mengembalikan `Option<u32>`; `None` berarti tidak ada diskon.
- `cetak_status` memakai `let else` untuk menangani transaksi yang belum dibayar.
- `total` memakai `match` pada `Option<u32>`.

Kontrak setelah Bab 6:

- Status dan metode bayar tidak boleh berupa string bebas.
- Diskon 10 persen tetap diberikan saat subtotal minimal 40.000.
- Array tiga item masih dipakai; `Vec` baru dibahas di Bab 8.
- Belum ada `Result` atau error handling untuk pembayaran.
- Metode bayar masih dipilih di kode, belum dibaca dari input.

### Keadaan setelah Bab 7

Bab 7 memecah Kasir Warung menjadi package dengan library crate dan binary crate. Logika pindah ke `src/lib.rs` dan module domain, sementara `src/main.rs` hanya mengatur alur terminal.

```text
kasir_warung/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── main.rs
    ├── menu.rs
    ├── transaksi.rs
    └── laporan.rs
```

Kontrak setelah Bab 7:

- `lib.rs` mengekspor `menu`, `transaksi`, dan `laporan` dengan `pub mod`.
- Binary memakai library lewat `use kasir_warung::...`.
- Tipe dan method yang dipakai lintas module diberi `pub`; detail internal tetap private.
- Output struk tidak berubah dari Bab 6.
- Masih memakai array tiga item. `Vec` baru muncul di Bab 8.
- Belum ada `Result` untuk error yang bisa dipulihkan.

### Keadaan setelah Bab 8

Bab 8 mengganti array tetap dengan collection dinamis. Katalog menu disimpan di `HashMap<String, MenuItem>`, sedangkan daftar pesanan disimpan di `Vec<ItemPesanan>`.

```rust
struct MenuItem {
    nama: String,
    harga: u32,
}

struct ItemPesanan {
    kode: String,
    jumlah: u32,
}

struct Transaksi {
    pelanggan: String,
    menu: HashMap<String, MenuItem>,
    items: Vec<ItemPesanan>,
}
```

Kontrak setelah Bab 8:

- Jumlah item tidak lagi dibatasi tiga.
- Harga dicari dari katalog menu berdasarkan kode.
- Item dengan kode yang tidak ada saat ini dilewati; belum ada error `Result`.
- `String` untuk nama menu dan kode item dimiliki oleh struct.
- Status dan metode pembayaran dari Bab 6 tetap ada di model lengkap.
- Belum ada penanganan error input dan lookup yang eksplisit.

### Keadaan setelah Bab 9

Bab 9 mengubah kegagalan input, lookup menu, dan pembayaran menjadi nilai `Result`. Panic tidak lagi dipakai untuk input pengguna.

```rust
#[derive(Debug)]
enum KasirError {
    InputTidakValid(String),
    ItemTidakDitemukan(String),
    PembayaranGagal(String),
}

fn parse_jumlah(input: &str) -> Result<usize, KasirError>;
fn subtotal(&self) -> Result<u32, KasirError>;
fn bayar(&mut self, metode: MetodeBayar) -> Result<(), KasirError>;
```

Kontrak setelah Bab 9:

- `KasirError` mengimplementasikan `Display` dan `Error`.
- `parse_jumlah` mengembalikan `Err` untuk input kosong, bukan angka, atau di luar rentang.
- Lookup menu memakai `ok_or_else` untuk menghasilkan `ItemTidakDitemukan`.
- `subtotal` dan `cetak` bisa mengembalikan `Result`.
- Metode bayar tetap enum dan disimpan sebagai `Option`; pembayaran kedua menghasilkan error.
- Panic hanya untuk invariant internal yang rusak, bukan input pengguna.
- `Vec` dan `HashMap` tetap dipakai.
- Belum ada handler HTTP; pemetaan error ke status HTTP masuk Capstone.

### Keadaan setelah Bab 10

Bab 10 menambahkan trait pembayaran, fungsi generik, dan view yang meminjam transaksi. Pembayaran sekarang memakai kontrak perilaku, bukan hanya enum metode.

```rust
trait PaymentMethod {
    fn label(&self) -> &'static str;
    fn bayar(&self, total: u32) -> Result<(), KasirError>;
}

fn proses_pembayaran<T: PaymentMethod>(
    metode: &T,
    total: u32,
) -> Result<(), KasirError>;

struct Ringkasan<'a> {
    pelanggan: &'a str,
    subtotal: u32,
}
```

Kontrak setelah Bab 10:

- `Tunai` dan `Qris` mengimplementasikan `PaymentMethod`.
- `proses_pembayaran` generik dengan trait bound.
- `Ringkasan<'a>` meminjam nama pelanggan dari `Transaksi`, tanpa clone.
- `KasirError` tetap menjadi tipe error aplikasi.
- Belum ada trait object `Box<dyn PaymentMethod>`; itu masuk Bab 18.
- Belum ada test otomatis; itu masuk Bab 11.

### Keadaan setelah Bab 11

Bab 11 menambahkan unit test di module dan integration test di folder `tests`. Perilaku aplikasi tidak berubah; test menjadi jaring pengaman untuk validasi, perhitungan, dan pembayaran.

```text
kasir_warung/
├── src/
│   ├── lib.rs
│   ├── menu.rs
│   ├── transaksi.rs
│   ├── laporan.rs
│   └── main.rs
└── tests/
    └── kasir.rs
```

Kontrak setelah Bab 11:

- Unit test memeriksa `parse_jumlah`, subtotal, diskon, dan pembayaran kedua.
- Integration test memakai API public library.
- Test error memakai `matches!` atau pengecekan `Result`.
- Perilaku runtime tetap sama.
- Belum ada test endpoint HTTP; endpoint baru ada di Capstone.
- Belum ada iterator/closure tingkat lanjut; CLI laporan sudah ada di Bab 12 dan iterator masuk Bab 13.

### Keadaan setelah Bab 12

Bab 12 menambahkan binary CLI laporan harian. Binary membaca path file dari argumen, membaca file dengan `fs::read_to_string`, memvalidasi setiap baris, lalu mencetak rekap ke stdout. Error ditulis ke stderr dan program keluar dengan status 1.

```text
kasir_warung/
├── src/
│   ├── lib.rs
│   ├── menu.rs
│   ├── transaksi.rs
│   ├── laporan.rs
│   └── main.rs
└── tests/
    ├── kasir.rs
    └── laporan.rs
```

Kontrak setelah Bab 12:

- Perintah: `kasir-laporan <file>`.
- Format baris: `KODE,jumlah`, satu baris per item; baris kosong diabaikan.
- `Config::build` menolak argumen yang kurang dengan `LaporanError::ArgumenKurang`.
- `jalankan` membaca file dan meneruskan error dengan `?`.
- `parse_baris` menormalkan kode menjadi huruf besar dan menolak jumlah nol atau bukan angka.
- `Rekap` memakai `HashMap<String, RingkasItem>` untuk jumlah dan subtotal per kode.
- Kode menu yang tidak dikenal menjadi `LaporanError::KodeTidakDikenal`.
- Laporan sukses ditulis dengan `println!`; error ditulis dengan `eprintln!`.
- Integration test di `tests/laporan.rs` menguji normalisasi, jumlah nol, kode tidak dikenal, dan total.
- Belum ada iterator dan closure tingkat lanjut; itu masuk Bab 13.

### Keadaan setelah Bab 13

Bab 13 menambahkan rekap transaksi berbasis closure dan iterator. Laporan menu dari Bab 12 tetap ada, lalu ditambah ringkasan pembayaran.

```text
kasir_warung/
├── src/
│   ├── lib.rs
│   ├── menu.rs
│   ├── transaksi.rs
│   ├── laporan.rs
│   ├── rangkuman.rs
│   └── main.rs
└── tests/
    ├── kasir.rs
    ├── laporan.rs
    └── rangkuman.rs
```

Kontrak setelah Bab 13:

- `TransaksiLaporan` adalah view ringkas berisi pelanggan, metode, total, dan status lunas.
- `total_lunas` memakai `filter`, `map`, dan `sum`.
- `total_lunas_fold` memakai `fold` untuk akumulasi.
- `total_per_metode` memakai `filter` dan `fold` dengan `HashMap<String, u32>`.
- `cetak_ringkasan` mengurutkan kunci metode sebelum mencetak agar output stabil.
- CLI tetap menulis laporan ke stdout dan error ke stderr.
- Test `rangkuman.rs` memeriksa total lunas dan rekap per metode.
- Belum ada iterator streaming yang mengembalikan hasil ke pemanggil; itu latihan lanjutan.

### Keadaan setelah Bab 14

Bab 14 memecah Kasir Warung menjadi workspace dengan library inti dan binary CLI. Root workspace memegang lockfile, folder target, dan profil release.

```text
kasir_warung/
├── Cargo.toml
├── Cargo.lock
├── kasir-core/
│   ├── Cargo.toml
│   └── src/lib.rs
├── kasir-cli/
│   ├── Cargo.toml
│   └── src/main.rs
└── target/
```

Kontrak setelah Bab 14:

- `kasir-core` memuat module menu, transaksi, laporan, dan rangkuman.
- `kasir-cli` bergantung pada `kasir-core` lewat `path = "../kasir-core"`.
- Satu `Cargo.lock` dan satu `target` dipakai bersama.
- `[profile.release] opt-level = 3` ditulis di root workspace.
- `cargo test -p kasir-core` dan `cargo run -p kasir-cli` dijalankan dari root.
- `cargo install --path kasir-cli` memasang binary lokal.
- `kasir-core` dan `kasir-cli` punya metadata publikasi masing-masing.
- Publikasi dan yank dilakukan per crate, bukan per workspace.
- Belum ada CI yang mem-publish otomatis; itu di luar cakupan bab.

### Keadaan setelah Bab 15

Bab 15 menambahkan cache menu single-thread ke `kasir-core`. Cache dibagikan dengan `Rc` dan dimutasi lewat `RefCell`, lalu batasnya diperiksa terhadap kebutuhan worker thread.

```text
kasir_warung/
├── Cargo.toml
├── Cargo.lock
├── kasir-core/
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── menu.rs
│       ├── transaksi.rs
│       ├── laporan.rs
│       ├── rangkuman.rs
│       └── cache.rs
├── kasir-cli/
│   ├── Cargo.toml
│   └── src/main.rs
└── target/
```

Kontrak setelah Bab 15:

- `MenuCache` menyimpan `HashMap<String, u32>` harga dan hit counter.
- `cari_harga` menerima `&RefCell<MenuCache>` dan memakai `borrow_mut`.
- `SharedMenu = Rc<RefCell<MenuCache>>` dipakai beberapa fungsi dalam satu thread.
- `Rc::strong_count` dipakai untuk memeriksa jumlah pemilik.
- `Rc<RefCell<T>>` tidak bisa dikirim ke thread lain; compiler menolak dengan `cannot be sent between threads safely`.
- CLI tetap single-thread.
- Rencana Bab 16: `Arc<Mutex<T>>` untuk cache dan state bersama lintas worker.
- Belum ada cache global lintas proses atau TTL; itu di luar cakupan.

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

`src/components/Sidebar.astro` adalah override Starlight yang menambahkan tanda selesai ke link sidebar. Script membaca kunci yang sama, menyesuaikan path dengan `import.meta.env.BASE_URL`, dan mendengarkan `rustbook:progress`. Grup bab menampilkan `2/5` saat sebagian selesai dan ikon centang saat semua halaman selesai. Kalau localStorage diblokir, sidebar tetap tampil normal tanpa tanda.

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
- [ ] Ada satu blok `Kalau di bahasa lain` yang singkat dan tidak memaksakan perbandingan.
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
