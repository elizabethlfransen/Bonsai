import { XMLParser } from "fast-xml-parser";

export const USER_AGENT =
  "Bonsai/1.0.0 (+https://github.com/elizabethlfransen/bonsai)";

export async function fetchJSON<T>(url: string, name?: string): Promise<T> {
  const response = await fetch(url, {
    method: "GET",
    headers: {
      "User-Agent": USER_AGENT,
      Accept: "application/json",
    },
  });
  if (!response.ok) {
    const message = `failed to download` + (name ? ` ${name}.` : ".") +
      ` Status: ${response.status} ${response.statusText}`;
    throw new Error(message);
  }
  const data = (await response.json()) as T;
  return data;
}

export async function fetchXML<T>(url: string, name?: string): Promise<T> {
  const response = await fetch(url, {
    method: "GET",
    headers: {
      "User-Agent": USER_AGENT,
      Accept: "application/xml",
    },
  });
  if (!response.ok) {
    const message = `failed to download` + (name ? ` ${name}.` : ".") +
      ` Status: ${response.status} ${response.statusText}`;
    throw new Error(message);
  }
  const data = new XMLParser({
        ignoreAttributes: false,
        trimValues: true
  }).parse(await response.text()) as T;
  return data;
}
