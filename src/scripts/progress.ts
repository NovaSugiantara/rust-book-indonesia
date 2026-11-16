const KEY = 'rustbook:progress:v1';

// Kunci progres disimpan tanpa base path supaya pindah domain/sub-path tidak menghapus progres.
const BASE = import.meta.env.BASE_URL;

export interface QuizProgress {
  correct: number;
  total: number;
  answers?: Record<string, number>;
}

export interface PageProgress {
  completed?: boolean;
  completedAt?: string;
  quiz?: QuizProgress;
}

export type ProgressMap = Record<string, PageProgress>;

export function pageKey(pathname: string): string {
  const clean = pathname.split('?')[0].split('#')[0];
  const path = clean.startsWith(BASE) ? `/${clean.slice(BASE.length)}` : clean;
  return path.endsWith('/') ? path : `${path}/`;
}

export function readProgress(): ProgressMap {
  try {
    const raw = localStorage.getItem(KEY);
    if (!raw) return {};
    const parsed = JSON.parse(raw);
    return parsed && typeof parsed === 'object' ? (parsed as ProgressMap) : {};
  } catch {
    return {};
  }
}

export function writeProgress(progress: ProgressMap): void {
  try {
    localStorage.setItem(KEY, JSON.stringify(progress));
  } catch {
    // localStorage bisa diblokir. Halaman tetap berjalan tanpa menyimpan progres.
  }
  window.dispatchEvent(new CustomEvent('rustbook:progress'));
}

export function patchPage(path: string, patch: Partial<PageProgress>): ProgressMap {
  const progress = readProgress();
  progress[path] = { ...progress[path], ...patch };
  writeProgress(progress);
  return progress;
}
