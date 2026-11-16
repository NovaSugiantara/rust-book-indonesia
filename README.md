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
- Sidebar untuk 21 bab buku Rust. Bab 1 sampai Bab 21 sudah lengkap; capstone dan proyek bonus tersedia sebagai materi terpisah.
- Bab 1: instalasi, Hello World, Cargo, dan studi kasus Kasir Warung.
- Bab 2: game tebak angka dengan input, crate rand, match, dan loop.
- Bab 3: variabel, tipe data, fungsi, komentar, dan control flow.
- Bab 4: ownership, borrow, reference, slice, dan aturan compiler.
- Bab 5: struct, method, associated function, dan Debug.
- Bab 6: enum, Option, match, if let, dan let else.
- Bab 7: package, crate, module, path, use, dan pemisahan file.
- Bab 8: Vec, String, HashMap, dan studi kasus collection.
- Bab 9: panic, Result, operator ?, dan error kustom.
- Bab 10: generic, trait, lifetime, dan perbandingan dengan interface TypeScript, Go, serta PHP.
- Bab 11: unit test, integration test, dan perbandingan dengan Jest, go test, PHPUnit, serta pytest.
- Bab 12: minigrep, argumen CLI, file, environment variable, stderr, dan CLI laporan harian Kasir Warung.
- Bab 13: closure, iterator, adapter lazy, perbaikan minigrep, dan perbandingan performa sesuai buku.
- Bab 14: release profile, publikasi crate, workspace, cargo install, dan custom command.
- Bab 15: Box, Deref, Drop, Rc, RefCell, reference cycle, Weak, dan cache menu single-thread.
- Bab 16: thread, channel, Mutex, Arc, Send/Sync, dan worker transaksi.
- Bab 17: future, async/await, task, stream, timeout, dan pembayaran async dengan padanan Tokio.
- Bab 18: encapsulation, trait object, dynamic dispatch, object safety, state pattern, dan registry pembayaran.
- Bab 19: pattern di match, let, if let, loop, destructuring, match guard, dan parser perintah.
- Bab 20: unsafe Rust, trait dan tipe lanjutan, function pointer, macro_rules, dan derive macro.
- Bab 21: server TCP manual, thread pool, graceful shutdown, dan jembatan ke Capstone.
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
