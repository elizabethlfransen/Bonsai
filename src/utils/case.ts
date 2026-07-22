/**
 * Transforms common folder naming styles (kebab, snake, camel)
 * into a clean, human-readable Title Case string.
 */
export function toTitleCase(folderName: string): string {
  if (!folderName) return "";

  // 1. Convert camelCase/PascalCase transitions into spaces (e.g., 'myFolder' -> 'my Folder')
  const spaceSeparated = folderName.replace(/([a-z])([A-Z])/g, "$1 $2");

  // 2. Split the string by dashes, underscores, or spaces
  const words = spaceSeparated.split(/[-_\s]+/);

  // 3. Capitalize the first letter of each word and lowercase the rest
  return words
    .filter((word) => word.length > 0) // Remove empty strings from consecutive separators
    .map((word) => word.charAt(0).toUpperCase() + word.slice(1).toLowerCase())
    .join(" ");
}
