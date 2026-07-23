import { z } from "zod";
import { MOD_LOADERS } from "@/modloaders.ts";
import * as TOML from "smol-toml";

export const ModpackSchema = z.object({
  name: z.string(),
  author: z.string(),
  version: z.string(),
  minecraft: z.object({
    version: z.string(),
  }),
  bonsai: z.object({
    version: z.enum(["1.0"]),
  }),
  modloader: z.object({
    id: z.enum(MOD_LOADERS.map((loader) => loader.id)),
    version: z.string(),
  }),
});

export type ModpackConfig = Omit<z.input<typeof ModpackSchema>, "bonsai">;

export function stringifyModpackConfig(config: ModpackConfig): string {
  return TOML.stringify(
    ModpackSchema.parse(Object.assign(config, { bonsai: { version: "1.0" } })),
  );
}

export async function writeModpack(
  config: ModpackConfig,
  relativePath: string,
) {
  await Deno.writeTextFile(relativePath, stringifyModpackConfig(config));
}

export async function readModpack(
  relativePath: string
) {
  return ModpackSchema.parse(TOML.parse(await Deno.readTextFile(relativePath)))
}