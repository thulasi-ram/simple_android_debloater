export const sleep = (ms: number) => new Promise((r) => setTimeout(r, ms));

export const isLastItem = (a: any[], i: number) => i == a.length - 1;
