import type { DialogFilter } from '@tauri-apps/api/dialog';

export const sleep = (ms: number) => new Promise((r) => setTimeout(r, ms));

export const isLastItem = (a: any[], i: number) => i == a.length - 1;

export const JSON_DIALOG_FILTER: DialogFilter = { name: 'JSON', extensions: ['json'] };
export const CSV_DIALOG_FILTER: DialogFilter = { name: 'JSON', extensions: ['json'] };
