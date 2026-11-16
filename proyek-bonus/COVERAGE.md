# Matriks cakupan Rust Book

Setiap baris menunjuk ke kode nyata di proyek ini. Konsep yang sengaja tidak dipakai ditandai `dijelaskan, tidak dipakai` beserta alasannya. Baris kosong tidak diperbolehkan.

| Konsep | Bab | File:baris | Kenapa dipakai di situ |
| --- | --- | --- | --- |
| Instalasi rustup, dijelaskan tidak dipakai | 1 | proyek-bonus/README.md:85 | Proyek menganggap Rust sudah terpasang dan langsung mulai dari Cargo. |
| Package Cargo, build, dan run | 1 | proyek-bonus/Cargo.toml:1, proyek-bonus/README.md:8 | Proyek adalah package Cargo yang dijalankan dengan cargo run. |
| rand, loop, dan match | 2 | proyek-bonus/src/shortcode.rs:49, proyek-bonus/src/worker.rs:14, proyek-bonus/src/stats.rs:13 | Kode pendek dibuat acak, worker berulang membaca channel, dan status dicocokkan dengan match. |
| Input terminal, dijelaskan tidak dipakai | 2 | proyek-bonus/README.md:86 | Layanan HTTP menerima input lewat JSON, bukan stdin terminal. |
| Variabel, shadowing, tipe, fungsi, dan control flow | 3 | proyek-bonus/src/config.rs:15, proyek-bonus/src/config.rs:16, proyek-bonus/src/stats.rs:29, proyek-bonus/src/stats.rs:13 | Konfigurasi memakai binding dan shadowing, fungsi menghitung statistik, dan if/match mengatur alur. |
| Ownership dan move | 4 | proyek-bonus/src/routes.rs:53, proyek-bonus/src/routes.rs:54 | Link dipindahkan ke store setelah dibuat, bukan dibiarkan dipakai bersama secara tidak sengaja. |
| Clone yang disengaja | 4 | proyek-bonus/src/routes.rs:53, proyek-bonus/src/main.rs:30 | Clone dipakai saat nilai memang dibutuhkan dua pemilik, bukan sebagai jalan pintas. |
| Reference dan borrow | 4 | proyek-bonus/src/routes.rs:45, proyek-bonus/src/stats.rs:21 | Handler meminjam request dan statistik meminjam field link tanpa menyalin data. |
| Slice | 4 | proyek-bonus/src/stats.rs:29, proyek-bonus/src/stats.rs:40 | Fungsi statistik menerima potongan daftar link, bukan memiliki seluruh collection. |
| Struct, method, dan associated function | 5 | proyek-bonus/src/model.rs:52, proyek-bonus/src/model.rs:60, proyek-bonus/src/model.rs:69 | Link adalah struct dengan constructor dan method untuk membaca status. |
| Enum, Option, dan match | 6 | proyek-bonus/src/model.rs:10, proyek-bonus/src/routes.rs:105, proyek-bonus/src/stats.rs:13 | Status link adalah enum, pencarian memakai Option, dan keputusan memakai match. |
| if let dan let else | 6 | proyek-bonus/src/routes.rs:47, proyek-bonus/src/routes.rs:74 | Kode kustom opsional memakai if let, dan link yang tidak ditemukan ditangani dengan let else. |
| Package, crate, module, pub, use, lib, dan main | 7 | proyek-bonus/src/lib.rs:8, proyek-bonus/src/lib.rs:18, proyek-bonus/src/main.rs:1 | Library mengekspor module, binary memakai library lewat use, dan lib.rs dipisahkan dari main.rs. |
| Vec, String, dan HashMap | 8 | proyek-bonus/src/store.rs:27, proyek-bonus/src/model.rs:54, proyek-bonus/src/stats.rs:44 | Store memakai HashMap, target link adalah String, dan daftar link diproses sebagai Vec. |
| panic vs Result, operator ?, dan custom error | 9 | proyek-bonus/src/error.rs:17, proyek-bonus/src/routes.rs:45, proyek-bonus/src/store.rs:109 | Error validasi dan database dikembalikan sebagai AppError, bukan panic. |
| Generic, trait, dan trait bound | 10 | proyek-bonus/src/stats.rs:29, proyek-bonus/src/store.rs:15 | sum_by generik atas T dan F, sementara Store membatasi implementasinya Send dan Sync. |
| Lifetime eksplisit | 10 | proyek-bonus/src/stats.rs:21, proyek-bonus/src/stats.rs:40 | StatsView meminjam code dari Link, jadi lifetime harus ditulis eksplisit. |
| Unit test | 11 | proyek-bonus/src/shortcode.rs:98, proyek-bonus/src/stats.rs:64, proyek-bonus/src/store.rs:183 | Test kecil menguji validasi kode, agregasi statistik, dan kedua implementasi store. |
| Integration test dan test endpoint | 11 | proyek-bonus/tests/api.rs:36, proyek-bonus/tests/api.rs:84 | Router diuji lewat request HTTP tanpa membuka port jaringan. |
| Environment dan argumen | 12 | proyek-bonus/src/config.rs:14, proyek-bonus/src/config.rs:40 | Alamat, database, dan mode store dibaca dari env, sedangkan --help dibaca dari argumen. |
| stderr untuk error | 12 | proyek-bonus/src/error.rs:56, proyek-bonus/src/main.rs:13 | Error internal dan bantuan CLI ditulis ke stderr, bukan stdout. |
| Closure dan iterator | 13 | proyek-bonus/src/stats.rs:37, proyek-bonus/src/stats.rs:41, proyek-bonus/src/stats.rs:44 | Total klik, ringkasan, dan top link memakai closure serta rantai iterator. |
| Profil release | 14 | proyek-bonus/Cargo.toml:20 | Profil release memakai LTO tipis dan satu codegen unit untuk binary akhir. |
| Dokumentasi dan doc test | 14 | proyek-bonus/src/shortcode.rs:13, proyek-bonus/src/shortcode.rs:15 | ShortCode punya doc comment dan contoh yang dijalankan cargo test. |
| Workspace Cargo, dijelaskan tidak dipakai | 14 | proyek-bonus/README.md:92 | Satu package dengan lib dan bin sudah cukup untuk proyek kecil ini. |
| Box | 15 | proyek-bonus/src/main.rs:19 | Store dipilih saat runtime lalu dibungkus `Box<dyn Store>` sebelum dibagikan. |
| Arc | 15 | proyek-bonus/src/main.rs:24, proyek-bonus/src/state.rs:10 | State dan trait object dibagi ke banyak handler serta task. |
| RwLock | 15 | proyek-bonus/src/store.rs:27 | Implementasi memory memakai RwLock agar pembacaan bisa bersamaan. |
| Mutex | 15 | proyek-bonus/src/state.rs:13 | Buffer audit dilindungi Mutex karena ditulis worker thread dan dibaca handler. |
| Deref | 15 | proyek-bonus/src/shortcode.rs:66 | ShortCode bisa dipakai sebagai str lewat Deref. |
| Drop | 15 | proyek-bonus/src/worker.rs:63 | AuditGuard menutup channel dan join thread saat shutdown. |
| Rc dan RefCell, dijelaskan tidak dipakai | 15 | proyek-bonus/README.md:87 | State lintas thread membutuhkan Arc dan lock yang Sync. |
| Weak, dijelaskan tidak dipakai | 15 | proyek-bonus/README.md:88 | Tidak ada kepemilikan melingkar yang perlu diputus. |
| std::thread | 16 | proyek-bonus/src/worker.rs:40 | Audit memakai thread blocking terpisah dari runtime async. |
| Channel | 16 | proyek-bonus/src/main.rs:26, proyek-bonus/src/worker.rs:41 | Klik dikirim ke worker lewat channel, bukan ditulis langsung dari handler. |
| Arc Mutex | 16 | proyek-bonus/src/worker.rs:30 | Buffer audit adalah state bersama yang dilindungi Arc dan Mutex. |
| Send dan Sync | 16 | proyek-bonus/src/main.rs:66, proyek-bonus/src/store.rs:15 | Ada assertion compile-time bahwa AppState bisa dipakai lintas task. |
| spawn_blocking, dijelaskan tidak dipakai | 16 | proyek-bonus/README.md:89 | Semua I/O database sudah async dan tidak ada kerja blocking di handler. |
| async dan await | 17 | proyek-bonus/src/routes.rs:41, proyek-bonus/src/worker.rs:13 | Handler dan worker menunggu operasi database serta channel secara async. |
| Future dan async trait | 17 | proyek-bonus/src/store.rs:18 | Method Store mengembalikan future lewat async trait. |
| Tokio task | 17 | proyek-bonus/src/worker.rs:13 | Worker klik dijalankan sebagai task Tokio terpisah. |
| Stream, dijelaskan tidak dipakai | 17 | proyek-bonus/README.md:90 | Response statistik kecil dan dikirim sebagai JSON biasa. |
| Trait object dyn Store | 18 | proyek-bonus/src/state.rs:10 | Handler memakai `Arc<dyn Store>` tanpa tahu backend memory atau SQLite. |
| State pattern, dijelaskan tidak dipakai | 18 | proyek-bonus/README.md:91 | Status link cukup enum; belum ada perilaku berbeda per state. |
| Pattern destructuring | 19 | proyek-bonus/src/store.rs:167 | Baris query SQL didestruktur menjadi code, target, clicks, dan status. |
| Pattern guard dan @ binding | 19 | proyek-bonus/src/stats.rs:14 | Label klik memakai binding @ dan guard untuk rentang nilai. |
| unsafe, dijelaskan tidak dipakai | 20 | proyek-bonus/src/lib.rs:6, proyek-bonus/README.md:93 | Crate memakai forbid(unsafe_code); kode web ini tidak membutuhkannya. |
| Associated type | 20 | proyek-bonus/src/store.rs:16 | Store mendefinisikan type Error yang harus dipenuhi implementasinya. |
| Newtype ShortCode | 20 | proyek-bonus/src/shortcode.rs:23 | Newtype menjaga validasi kode dan memberi tipe berbeda dari String biasa. |
| Display dan FromStr | 20 | proyek-bonus/src/shortcode.rs:80, proyek-bonus/src/shortcode.rs:86 | Kode bisa dicetak dan di-parse lewat trait standar. |
| Function pointer | 20 | proyek-bonus/src/routes.rs:125 | Validator URL diterima sebagai fn pointer agar bisa diganti tanpa generic. |
| macro_rules | 20 | proyek-bonus/src/error.rs:9, proyek-bonus/src/shortcode.rs:31 | Makro bad_request membuat error validasi dengan format string. |
| Thread pool dan penggantinya | 21 | proyek-bonus/README.md:94 | Tokio multi-thread dan axum menggantikan thread pool manual Bab 21. |
