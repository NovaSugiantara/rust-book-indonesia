export interface ChapterPage {
  label: string;
  slug: string;
}

export interface Chapter {
  number: number;
  title: string;
  slug: string;
  pages: ChapterPage[];
}

/**
 * Satu sumber untuk sidebar dan ringkasan progres.
 * Bab yang belum ditulis tetap punya satu halaman "Segera hadir".
 */
export const chapters: Chapter[] = [
  {
    number: 1,
    title: 'Memulai',
    slug: 'bab-1',
    pages: [
      { label: 'Ringkasan bab', slug: 'bab-1' },
      { label: '1.1 Instalasi', slug: 'bab-1/instalasi' },
      { label: '1.2 Hello, World!', slug: 'bab-1/hello-world' },
      { label: '1.3 Hello, Cargo!', slug: 'bab-1/hello-cargo' },
      { label: '1.4 Studi kasus: Kasir Warung', slug: 'bab-1/studi-kasus' },
    ],
  },
  {
    number: 2,
    title: 'Membuat game tebak angka',
    slug: 'bab-2',
    pages: [
      { label: 'Ringkasan bab', slug: 'bab-2' },
      { label: '2.1 Menyiapkan proyek', slug: 'bab-2/persiapan' },
      { label: '2.2 Membaca input', slug: 'bab-2/input' },
      { label: '2.3 Angka rahasia', slug: 'bab-2/angka-rahasia' },
      { label: '2.4 Membandingkan tebakan', slug: 'bab-2/membandingkan' },
      { label: '2.5 Loop dan input valid', slug: 'bab-2/loop' },
      { label: '2.6 Studi kasus: Kasir Warung', slug: 'bab-2/studi-kasus' },
    ],
  },
  {
    number: 3,
    title: 'Konsep dasar pemrograman',
    slug: 'bab-3',
    pages: [
      { label: 'Ringkasan bab', slug: 'bab-3' },
      { label: '3.1 Variabel dan mutability', slug: 'bab-3/variabel-mutability' },
      { label: '3.2 Tipe data', slug: 'bab-3/tipe-data' },
      { label: '3.3 Fungsi', slug: 'bab-3/fungsi' },
      { label: '3.4 Komentar', slug: 'bab-3/komentar' },
      { label: '3.5 Control flow', slug: 'bab-3/control-flow' },
      { label: '3.6 Studi kasus: Kasir Warung', slug: 'bab-3/studi-kasus' },
    ],
  },
  {
    number: 4,
    title: 'Memahami ownership',
    slug: 'bab-4',
    pages: [
      { label: 'Ringkasan bab', slug: 'bab-4' },
      { label: '4.1 Apa itu ownership', slug: 'bab-4/what-is-ownership' },
      { label: '4.2 Reference dan borrowing', slug: 'bab-4/references-borrowing' },
      { label: '4.3 Slice', slug: 'bab-4/slices' },
      { label: '4.4 Studi kasus: Kasir Warung', slug: 'bab-4/studi-kasus' },
    ],
  },
  {
    number: 5,
    title: 'Menggunakan struct',
    slug: 'bab-5',
    pages: [
      { label: 'Ringkasan bab', slug: 'bab-5' },
      { label: '5.1 Mendefinisikan dan membuat instance', slug: 'bab-5/defining-structs' },
      { label: '5.2 Contoh program dengan struct', slug: 'bab-5/example-structs' },
      { label: '5.3 Method', slug: 'bab-5/method-syntax' },
      { label: '5.4 Studi kasus: Kasir Warung', slug: 'bab-5/studi-kasus' },
    ],
  },
  {
    number: 6,
    title: 'Enum dan pattern matching',
    slug: 'bab-6',
    pages: [
      { label: 'Ringkasan bab', slug: 'bab-6' },
      { label: '6.1 Mendefinisikan enum', slug: 'bab-6/defining-enums' },
      { label: '6.2 Match', slug: 'bab-6/match' },
      { label: '6.3 If let dan let else', slug: 'bab-6/if-let' },
      { label: '6.4 Studi kasus: Kasir Warung', slug: 'bab-6/studi-kasus' },
    ],
  },
  {
    number: 7,
    title: 'Package, crate, dan module',
    slug: 'bab-7',
    pages: [
      { label: 'Ringkasan bab', slug: 'bab-7' },
      { label: '7.1 Package dan crate', slug: 'bab-7/packages-crates' },
      { label: '7.2 Module', slug: 'bab-7/modules' },
      { label: '7.3 Path', slug: 'bab-7/paths' },
      { label: '7.4 Use', slug: 'bab-7/use' },
      { label: '7.5 Memisahkan module ke file', slug: 'bab-7/files' },
      { label: '7.6 Studi kasus: Kasir Warung', slug: 'bab-7/studi-kasus' },
    ],
  },
  {
    number: 8,
    title: 'Collection umum',
    slug: 'bab-8',
    pages: [
      { label: 'Ringkasan bab', slug: 'bab-8' },
      { label: '8.1 Vector', slug: 'bab-8/vectors' },
      { label: '8.2 String', slug: 'bab-8/strings' },
      { label: '8.3 HashMap', slug: 'bab-8/hash-maps' },
      { label: '8.4 Studi kasus: Kasir Warung', slug: 'bab-8/studi-kasus' },
    ],
  },
  {
    number: 9,
    title: 'Error handling',
    slug: 'bab-9',
    pages: [
      { label: 'Ringkasan bab', slug: 'bab-9' },
      { label: '9.1 Panic', slug: 'bab-9/panic' },
      { label: '9.2 Result', slug: 'bab-9/result' },
      { label: '9.3 Panic atau Result', slug: 'bab-9/panic-or-result' },
      { label: '9.4 Studi kasus: Kasir Warung', slug: 'bab-9/studi-kasus' },
    ],
  },
  {
    number: 10,
    title: 'Generic, trait, dan lifetime',
    slug: 'bab-10',
    pages: [
      { label: 'Ringkasan bab', slug: 'bab-10' },
      { label: '10.1 Generic', slug: 'bab-10/generics' },
      { label: '10.2 Trait', slug: 'bab-10/traits' },
      { label: '10.3 Lifetime', slug: 'bab-10/lifetimes' },
      { label: '10.4 Studi kasus: Kasir Warung', slug: 'bab-10/studi-kasus' },
    ],
  },
  {
    number: 11,
    title: 'Menulis test otomatis',
    slug: 'bab-11',
    pages: [
      { label: 'Ringkasan bab', slug: 'bab-11' },
      { label: '11.1 Menulis test', slug: 'bab-11/writing-tests' },
      { label: '11.2 Menjalankan test', slug: 'bab-11/running-tests' },
      { label: '11.3 Organisasi test', slug: 'bab-11/test-organization' },
      { label: '11.4 Studi kasus: Kasir Warung', slug: 'bab-11/studi-kasus' },
    ],
  },
  {
    number: 12,
    title: 'Proyek I/O: membangun CLI',
    slug: 'bab-12',
    pages: [
      { label: 'Ringkasan bab', slug: 'bab-12' },
      { label: '12.1 Menerima argumen', slug: 'bab-12/accepting-arguments' },
      { label: '12.2 Membaca file', slug: 'bab-12/reading-file' },
      { label: '12.3 Refactoring dan error', slug: 'bab-12/refactoring' },
      { label: '12.4 TDD pencarian', slug: 'bab-12/tdd' },
      { label: '12.5 Environment variable', slug: 'bab-12/environment-variables' },
      { label: '12.6 Standard error', slug: 'bab-12/stderr' },
      { label: '12.7 Studi kasus: laporan harian', slug: 'bab-12/studi-kasus' },
    ],
  },
  {
    number: 13,
    title: 'Iterator dan closure',
    slug: 'bab-13',
    pages: [
      { label: 'Ringkasan bab', slug: 'bab-13' },
      { label: '13.1 Closure', slug: 'bab-13/closures' },
      { label: '13.2 Iterator', slug: 'bab-13/iterators' },
      { label: '13.3 Memperbaiki proyek I/O', slug: 'bab-13/improving-io' },
      { label: '13.4 Performance loop dan iterator', slug: 'bab-13/performance' },
      { label: '13.5 Studi kasus: rekap transaksi', slug: 'bab-13/studi-kasus' },
    ],
  },
  {
    number: 14,
    title: 'Cargo dan crates.io lebih lanjut',
    slug: 'bab-14',
    pages: [
      { label: 'Ringkasan bab', slug: 'bab-14' },
      { label: '14.1 Release profile', slug: 'bab-14/release-profiles' },
      { label: '14.2 Publikasi crate', slug: 'bab-14/publishing' },
      { label: '14.3 Workspace', slug: 'bab-14/workspaces' },
      { label: '14.4 cargo install', slug: 'bab-14/cargo-install' },
      { label: '14.5 Custom command', slug: 'bab-14/custom-commands' },
      { label: '14.6 Studi kasus: workspace', slug: 'bab-14/studi-kasus' },
    ],
  },
  {
    number: 15,
    title: 'Smart pointer',
    slug: 'bab-15',
    pages: [
      { label: 'Ringkasan bab', slug: 'bab-15' },
      { label: '15.1 Box', slug: 'bab-15/box' },
      { label: '15.2 Deref', slug: 'bab-15/deref' },
      { label: '15.3 Drop', slug: 'bab-15/drop' },
      { label: '15.4 Rc', slug: 'bab-15/rc' },
      { label: '15.5 Interior mutability', slug: 'bab-15/interior-mutability' },
      { label: '15.6 Reference cycle dan Weak', slug: 'bab-15/reference-cycles' },
      { label: '15.7 Studi kasus: cache menu', slug: 'bab-15/studi-kasus' },
    ],
  },
  {
    number: 16,
    title: 'Concurrency tanpa rasa takut',
    slug: 'bab-16',
    pages: [
      { label: 'Ringkasan bab', slug: 'bab-16' },
      { label: '16.1 Thread', slug: 'bab-16/threads' },
      { label: '16.2 Message passing', slug: 'bab-16/message-passing' },
      { label: '16.3 Shared state', slug: 'bab-16/shared-state' },
      { label: '16.4 Send dan Sync', slug: 'bab-16/send-sync' },
      { label: '16.5 Studi kasus: worker transaksi', slug: 'bab-16/studi-kasus' },
    ],
  },
  {
    number: 17,
    title: 'Dasar pemrograman async',
    slug: 'bab-17',
    pages: [
      { label: 'Ringkasan bab', slug: 'bab-17' },
      { label: '17.1 Future dan sintaks async', slug: 'bab-17/futures-syntax' },
      { label: '17.2 Concurrency dengan async', slug: 'bab-17/concurrency-with-async' },
      { label: '17.3 Banyak future dan timeout', slug: 'bab-17/more-futures' },
      { label: '17.4 Stream', slug: 'bab-17/streams' },
      { label: '17.5 Trait untuk async', slug: 'bab-17/traits-for-async' },
      { label: '17.6 Future, task, dan thread', slug: 'bab-17/futures-tasks-threads' },
      { label: '17.7 Studi kasus: pembayaran async', slug: 'bab-17/studi-kasus' },
    ],
  },
  {
    number: 18,
    title: 'Fitur berorientasi objek',
    slug: 'bab-18',
    pages: [
      { label: 'Ringkasan bab', slug: 'bab-18' },
      { label: '18.1 Karakteristik OOP', slug: 'bab-18/what-is-oo' },
      { label: '18.2 Trait object', slug: 'bab-18/trait-objects' },
      { label: '18.3 State pattern', slug: 'bab-18/design-patterns' },
      { label: '18.4 Studi kasus: registry pembayaran', slug: 'bab-18/studi-kasus' },
    ],
  },
  {
    number: 19,
    title: 'Pattern dan matching',
    slug: 'bab-19',
    pages: [
      { label: 'Ringkasan bab', slug: 'bab-19' },
      { label: '19.1 Tempat memakai pattern', slug: 'bab-19/places' },
      { label: '19.2 Refutability', slug: 'bab-19/refutability' },
      { label: '19.3 Sintaks pattern', slug: 'bab-19/syntax' },
      { label: '19.4 Studi kasus: parser perintah', slug: 'bab-19/studi-kasus' },
    ],
  },
  {
    number: 20,
    title: 'Fitur lanjutan',
    slug: 'bab-20',
    pages: [
      { label: 'Ringkasan bab', slug: 'bab-20' },
      { label: '20.1 Unsafe Rust', slug: 'bab-20/unsafe' },
      { label: '20.2 Trait lanjutan', slug: 'bab-20/advanced-traits' },
      { label: '20.3 Tipe lanjutan', slug: 'bab-20/advanced-types' },
      { label: '20.4 Fungsi dan closure lanjutan', slug: 'bab-20/advanced-functions' },
      { label: '20.5 Macro', slug: 'bab-20/macros' },
      { label: '20.6 Studi kasus: validasi dan error response', slug: 'bab-20/studi-kasus' },
    ],
  },
  {
    number: 21,
    title: 'Proyek akhir: web server multithread',
    slug: 'bab-21',
    pages: [
      { label: 'Ringkasan bab', slug: 'bab-21' },
      { label: '21.1 Server single-thread', slug: 'bab-21/single-thread' },
      { label: '21.2 Thread pool', slug: 'bab-21/multithread' },
      { label: '21.3 Graceful shutdown', slug: 'bab-21/graceful-shutdown' },
      { label: '21.4 Studi kasus: server Kasir Warung', slug: 'bab-21/studi-kasus' },
    ],
  },
];

export const capstonePages: ChapterPage[] = [
  { label: 'Ringkasan Capstone', slug: 'capstone' },
  { label: 'Memilih framework', slug: 'capstone/framework' },
  { label: 'Langkah 1: workspace dan health', slug: 'capstone/langkah-1-persiapan' },
  { label: 'Langkah 2: CRUD menu', slug: 'capstone/langkah-2-menu' },
  { label: 'Langkah 3: pesanan dan validasi', slug: 'capstone/langkah-3-transaksi' },
  { label: 'Langkah 4: pembayaran dan status', slug: 'capstone/langkah-4-pembayaran-status' },
  { label: 'Langkah 5: laporan dan background task', slug: 'capstone/langkah-5-laporan-background' },
  { label: 'Langkah 6: test dan release', slug: 'capstone/langkah-6-test-release' },
  { label: 'Menjalankan dan deploy', slug: 'capstone/menjalankan' },
];

export const bonusPages: ChapterPage[] = [
  { label: 'Pengantar proyek', slug: 'proyek-bonus' },
  { label: 'Langkah 1: model dan error', slug: 'proyek-bonus/langkah-1' },
  { label: 'Langkah 2: store dan worker', slug: 'proyek-bonus/langkah-2' },
  { label: 'Langkah 3: HTTP API', slug: 'proyek-bonus/langkah-3' },
  { label: 'Langkah 4: test dan release', slug: 'proyek-bonus/langkah-4' },
];

export const totalPages = chapters.reduce((total, chapter) => total + chapter.pages.length, 0);
