import { defineCommand } from "@/utils/commands.ts";
import { readModpack as readModpackConfig } from "@/schemas/modpack.ts";
import { MODRINTH_CLIENT } from "@/api/modrinth.ts";
import { select } from "@inquirer/prompts";
import { version } from "yargs";
import { loadWithSpinner } from "@/utils/spinners.ts";
import ora from "ora";

export default defineCommand({
  command: "add-mod <mod>",
  describe: "add a mod",
  builder: (yargs) =>
    yargs
      .positional("mod", {
        type: "string",
        describe: "mod to search for on modrinth.",
      }),
  handler: async (argv) => {
    const modpack = await loadWithSpinner(
      readModpackConfig(argv.packFile),
      "modpack config",
    );
    const modOptions = (await (async () => {
      const spinner = ora("Searching for mod").start();
      try {
        const result = await MODRINTH_CLIENT.labrinth.projects_v2.search({
          query: argv.mod,
          facets: [
            ["project_type:mod"],
            [`versions:${modpack.minecraft.version}`],
            [`categories:${modpack.modloader.id}`],
          ],
        });
        spinner.stop();
        return result;
      } catch (e) {
        spinner.fail("Fail to load search results");
        throw e;
      }
    })()).hits;

    if (modOptions.length == 0) {
      console.error("No mods found");
      process.exit(1);
    }
    const mod = modOptions.length == 1 ? modOptions[0] : await select({
      message: "Search Results",
      choices: modOptions.map((item) => ({
        name: item.title,
        value: item,
      })),
    });
    const versionOptions = await loadWithSpinner(
      MODRINTH_CLIENT.labrinth.versions_v3
        .getProjectVersions(mod.project_id, {
          game_versions: [modpack.minecraft.version],
          loaders: [modpack.modloader.id],
        }),
      "mod versions",
    );
    if (versionOptions.length == 0) {
      console.error("No version found");
      process.exit(1);
    }
    const version = versionOptions.length == 1
      ? versionOptions[0]
      : await select({
        message: "Available Versions",
        choices: versionOptions.map((item) => ({
          name: item.name,
          value: item,
        })),
      });
    console.log(`${mod.title} - ${version.version_number}`);
  },
});
``;
