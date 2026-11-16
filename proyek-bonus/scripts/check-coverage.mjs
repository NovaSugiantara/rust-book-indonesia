import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const here = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(here, '../..');
const coveragePath = path.join(repoRoot, 'proyek-bonus/COVERAGE.md');
const text = fs.readFileSync(coveragePath, 'utf8');

const rows = text
  .split('\n')
  .filter((line) => line.startsWith('|'))
  .map((line) => line.split('|').slice(1, -1).map((cell) => cell.trim()))
  .filter((cells) => cells.length === 4 && !cells[0].startsWith('---') && cells[0] !== 'Konsep');

for (const cells of rows) {
  if (cells.some((cell) => cell.length === 0)) {
    throw new Error(`Baris matriks punya sel kosong: ${cells.join(' | ')}`);
  }
}

const refs = [...text.matchAll(/proyek-bonus\/[^\s,;|]+:\d+/g)].map((match) => match[0]);
if (refs.length === 0) throw new Error('Tidak ada referensi file:baris di COVERAGE.md');

for (const ref of refs) {
  const [relativePath, lineText] = ref.split(/:(?=\d+$)/);
  const fullPath = path.join(repoRoot, relativePath);
  if (!fs.existsSync(fullPath)) throw new Error(`File tidak ada: ${ref}`);

  const line = Number(lineText);
  const lineCount = fs.readFileSync(fullPath, 'utf8').split('\n').length;
  if (line < 1 || line > lineCount) {
    throw new Error(`Baris tidak ada: ${ref} (file punya ${lineCount} baris)`);
  }
}

console.log(`Coverage OK: ${rows.length} baris matriks, ${refs.length} referensi file:baris valid.`);
