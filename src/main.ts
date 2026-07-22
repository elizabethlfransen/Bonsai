// @deno-types="npm:@types/yargs@^17"
import yargs from "yargs";
import minecraftVersions from "./bin/utils/minecraft-versions.ts";
import { MOD_LOADERS } from "./modloaders.ts";

await yargs(Deno.args)
  .scriptName("bonsai")
  .usage("$0 <command> [options]")
  .command(
    "utils",
    "Utility sub commands for printing data",
    (yargs) =>
      yargs.command(
        "minecraft-versions",
        "Prints minecraft versions",
        minecraftVersions,
      ).command(
        "modloaders <mcVersion>",
        "Prints mod loaders",
        (yargs) =>
          yargs
            .positional("mcVersion", {
              type: "string",
              describe: 'Minecraft version to query mod loader versions for',
              demandOption: true,
            }),
        async ({mcVersion}) => {
          await Promise.all(
            MOD_LOADERS.map(async (
              loader,
            ) => [loader.name, await loader.getVersionList(mcVersion)] as const),
          ).then(loaders => loaders.forEach(([loader, versions]) => console.log(`${loader} - ${versions.latest}`)));
        },
      ),
  )
  .demandCommand(1, "You need at least one command before moving on.")
  .help()
  .parseAsync();
