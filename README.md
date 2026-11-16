# Rust dari Nol

Situs belajar Rust berbahasa Indonesia yang statis dan interaktif. Materinya adaptasi dari [The Rust Programming Language](https://doc.rust-lang.org/stable/book/) dengan penjelasan ulang, playground, kuis, dan studi kasus Kasir Warung.

## Menjalankan proyek

```console
$ npm install
$ npm run dev
```

Buka alamat yang ditampilkan Astro, biasanya `http://localhost:4321`. Untuk memeriksa hasil produksi:

```console
$ npm run build
$ npm run preview
```

## Deploy

Situs di-deploy otomatis ke GitHub Pages oleh `.github/workflows/deploy.yml` setiap push ke `main` (atau lewat `workflow_dispatch` manual).

Alamat produksi: <https://novasugiantara.github.io/rust-book-indonesia/>. Ini project page, jadi `site` dan `base` di `astro.config.ts` sudah diisi `https://novasugiantara.github.io` dan `/rust-book-indonesia`.

Sekali saja, di repo GitHub: **Settings > Pages > Source: GitHub Actions**.

Aturan link internal saat menambah halaman:

- di `.astro`/`.ts` pakai `import.meta.env.BASE_URL`, jangan tulis `/` di awal href;
- di `.md`/`.mdx` pakai link relatif (contoh `../bab-1/` atau `instalasi/`), karena link absolut seperti `/bab-1/` tidak ikut mendapat `base`.

## Isi saat ini

- Halaman depan berbahasa Indonesia dengan ringkasan progres.
- Sidebar untuk 21 bab buku Rust. Bab 1, Bab 2, dan Bab 3 sudah lengkap, bab lain masih halaman "Segera hadir".
- Bab 1: instalasi, Hello World, Cargo, dan studi kasus Kasir Warung.
- Bab 2: game tebak angka dengan input, crate rand, match, dan loop.
- Bab 3: variabel, tipe data, fungsi, komentar, dan control flow.
- Capstone: halaman rencana API Kasir Warung dengan axum, Tokio, dan SQLite, plus perbandingan axum, actix-web, dan rocket. Detail implementasi menyusul setelah Bab 21.
- Proyek Bonus: URL shortener dengan statistik klik, kode lengkap di `proyek-bonus/`, halaman pengantar dan empat langkah di situs.
- Komponen playground, kuis, studi kasus, dan penanda progres. Semuanya berjalan tanpa backend.

## Stack

- Astro dan Starlight
- TypeScript
- npm
- Tidak ada database atau API sendiri. Satu-satunya permintaan jaringan dari halaman adalah ke playground Rust publik saat tombol Jalankan diklik.

## Struktur penting

```text
src/
├── components/          Komponen interaktif dan footer
├── content/docs/        Halaman Starlight
├── data/chapters.ts     Daftar bab untuk sidebar dan progres
├── scripts/progress.ts  Baca dan tulis progres di localStorage
└── styles/custom.css    Warna aksen dan gaya halaman depan

proyek-bonus/            Crate Rust terpisah: URL shortener dengan axum
```

`proyek-bonus/` tidak ikut build Astro. Folder `target/` dan file database di dalamnya diabaikan Git.

Panduan menulis bab, aturan gaya, dan keadaan studi kasus ada di [CONTRIBUTING.md](./CONTRIBUTING.md).

## Lisensi dan atribusi

Situs ini adaptasi dari [The Rust Programming Language](https://doc.rust-lang.org/stable/book/) oleh Steve Klabnik, Carol Nichols, dan Chris Krycho, dengan kontribusi dari komunitas Rust. Buku aslinya dilisensikan MIT atau Apache-2.0. Kode situs ini mengikuti lisensi yang sama kecuali dinyatakan lain.
