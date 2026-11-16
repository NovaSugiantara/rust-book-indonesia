import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';
import { chapters } from './src/data/chapters';

const sidebar = chapters.map((chapter) =>
  chapter.number === 1
    ? {
        label: `${chapter.number}. ${chapter.title}`,
        items: chapter.pages.map((page) => ({ label: page.label, link: `/${page.slug}/` })),
      }
    : { label: `${chapter.number}. ${chapter.title}`, link: `/${chapter.slug}/` }
);

export default defineConfig({
  // GitHub Pages project page. `base` sengaja tanpa trailing slash, sesuai aturan Astro;
  // semua link internal di kode memakai import.meta.env.BASE_URL yang berakhiran slash.
  site: 'https://novasugiantara.github.io',
  base: '/rust-book-indonesia',
  trailingSlash: 'always',
  integrations: [
    starlight({
      title: 'Rust dari Nol',
      description:
        'Belajar Rust dalam bahasa Indonesia, dari instalasi sampai proyek nyata. Adaptasi The Rust Programming Language.',
      logo: { src: './src/assets/logo.svg', alt: '' },
      favicon: '/favicon.svg',
      defaultLocale: 'root',
      locales: {
        root: { label: 'Bahasa Indonesia', lang: 'id' },
        en: { label: 'English', lang: 'en' },
      },
      customCss: ['./src/styles/custom.css'],
      components: {
        Footer: './src/components/Footer.astro',
      },
      credits: true,
      pagination: true,
      disable404Route: true,
      lastUpdated: false,
      sidebar,
      tableOfContents: { minHeadingLevel: 2, maxHeadingLevel: 3 },
    }),
  ],
});
