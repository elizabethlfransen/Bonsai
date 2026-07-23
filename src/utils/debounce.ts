import sleep from "@/utils/sleep.ts";

export default async function debounce<T>(
  fn: () => Promise<T>,
  abortSignal: AbortSignal,
  delay: number = 300,
) {
  await sleep(delay, abortSignal);
  if (!abortSignal.aborted) {
    return await fn();
  }
}
