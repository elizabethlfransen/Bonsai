import { defineCommand } from "@/utils/commands.ts";
import { toTitleCase } from "@/utils/case.ts";
import { basename } from "@std/path";
import { input, search, select } from "@inquirer/prompts";
import { loadWithSpinner } from "@/utils/spinners.ts";
import { getMinecraftVersions } from "@/minecraft-version.ts";
import { MOD_LOADERS } from "@/modloaders.ts";
import { stringifyModpackConfig, writeModpack } from "@/schemas/modpack.ts";

export default defineCommand({
  command: "init",
  describe: "Initialize a modpack project",
  builder: (yargs) =>
    yargs.option("name", {
      type: "string",
      describe: "The of the project",
    }).option("author", {
      type: "string",
      describe: "The author of the project",
    }).option("modpack-version", {
      type: "string",
      describe: "The version of the modpack",
    }).option("minecraft-version", {
      type: "string",
      describe: "The version of minecraft the modpack is for",
    }).option("modloader", {
      type: "string",
      describe: "The modloader to use",
    }).option("modloader-version", {
      type: "string",
      describe: "The version of modloader to use",
    }),
  handler: async (argv) => {
    const name = argv.name ??
      await input({
        message: "Modpack name",
        default: toTitleCase(basename(Deno.cwd())),
      });
    const author = argv.author ?? await input({
      message: "Author",
      default: getUserName(),
    });
    const modpackVersion = argv.modpackVersion ?? await input({
      message: "Modpack Version",
      default: "0.1.0",
    });
    const minecraftVersions = await loadWithSpinner(
      getMinecraftVersions(),
      "minecraft versions",
    );
    const minecraftVersion = argv.minecraftVersion ?? await search({
      message: "Minecraft Version",
      default: minecraftVersions.latest.release,
      source: (str) =>
        minecraftVersions.versions.map((version) => version.id).filter((v) =>
          !str || v.startsWith(str)
        ),
    });
    if (
      !minecraftVersions.versions.find((validVersion) =>
        validVersion.id == minecraftVersion
      )
    ) {
      console.error("Invalid minecraft version");
      process.exit(1);
    }
    const loaderOrLoaderId = argv.modloader ?? await select({
      message: "Modloader",
      choices: MOD_LOADERS.map((loader) => ({
        name: loader.name,
        value: loader,
      })),
    });
    const modloader = typeof loaderOrLoaderId === "string"
      ? MOD_LOADERS.find((loader) => loader.id = loaderOrLoaderId)
      : loaderOrLoaderId;
    if (!modloader) {
      console.error("Invalid modloader");
      process.exit(1);
    }
    const loaderVersions = await loadWithSpinner(
      modloader.getVersionList(minecraftVersion),
      `${modloader.name} versions`,
    );
    const modloaderVersion = argv.modloaderVersion ?? await search({
      message: `${modloader.name} version`,
      source: () => loaderVersions.versions,
      default: loaderVersions.latest,
    });
    await loadWithSpinner(
      writeModpack({
        name,
        author,
        version: modpackVersion,
        minecraft: {
            version: minecraftVersion
        },
        modloader: {
          id: modloader.id,
          version: modloaderVersion,
        },
      }, argv.packFile),
      "Writing",
      "Wrote",
      "modpack config",
    );
  },
});

function getUserName(): string | undefined {
  const result = Deno.env.get("USERNAME") || Deno.env.get("USER") ||
    Deno.env.get("LOGNAME");
  if (!result) return result;
  return toTitleCase(result);
}
