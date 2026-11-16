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
  { number: 10, title: 'Generic, trait, dan lifetime', slug: 'bab-10', pages: [{ label: 'Ringkasan bab', slug: 'bab-10' }] },
  { number: 11, title: 'Menulis test otomatis', slug: 'bab-11', pages: [{ label: 'Ringkasan bab', slug: 'bab-11' }] },
  { number: 12, title: 'Proyek I/O: membangun CLI', slug: 'bab-12', pages: [{ label: 'Ringkasan bab', slug: 'bab-12' }] },
  { number: 13, title: 'Iterator dan closure', slug: 'bab-13', pages: [{ label: 'Ringkasan bab', slug: 'bab-13' }] },
  { number: 14, title: 'Cargo dan crates.io lebih lanjut', slug: 'bab-14', pages: [{ label: 'Ringkasan bab', slug: 'bab-14' }] },
  { number: 15, title: 'Smart pointer', slug: 'bab-15', pages: [{ label: 'Ringkasan bab', slug: 'bab-15' }] },
  { number: 16, title: 'Concurrency tanpa rasa takut', slug: 'bab-16', pages: [{ label: 'Ringkasan bab', slug: 'bab-16' }] },
  { number: 17, title: 'Dasar pemrograman async', slug: 'bab-17', pages: [{ label: 'Ringkasan bab', slug: 'bab-17' }] },
  { number: 18, title: 'Fitur berorientasi objek', slug: 'bab-18', pages: [{ label: 'Ringkasan bab', slug: 'bab-18' }] },
  { number: 19, title: 'Pattern dan matching', slug: 'bab-19', pages: [{ label: 'Ringkasan bab', slug: 'bab-19' }] },
  { number: 20, title: 'Fitur lanjutan', slug: 'bab-20', pages: [{ label: 'Ringkasan bab', slug: 'bab-20' }] },
  { number: 21, title: 'Proyek akhir: web server multithread', slug: 'bab-21', pages: [{ label: 'Ringkasan bab', slug: 'bab-21' }] },
];

export const capstonePages: ChapterPage[] = [
  { label: 'Ringkasan Capstone', slug: 'capstone' },
  { label: 'Memilih framework', slug: 'capstone/framework' },
];

export const bonusPages: ChapterPage[] = [
  { label: 'Pengantar proyek', slug: 'proyek-bonus' },
  { label: 'Langkah 1: model dan error', slug: 'proyek-bonus/langkah-1' },
  { label: 'Langkah 2: store dan worker', slug: 'proyek-bonus/langkah-2' },
  { label: 'Langkah 3: HTTP API', slug: 'proyek-bonus/langkah-3' },
  { label: 'Langkah 4: test dan release', slug: 'proyek-bonus/langkah-4' },
];

export const totalPages = chapters.reduce((total, chapter) => total + chapter.pages.length, 0);
