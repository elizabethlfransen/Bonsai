import { ArgumentsCamelCase, Argv } from "yargs";

export type GlobalOptionsArgs = ArgsOf<ReturnType<typeof applyGlobalOptions>>;
export type GlobalOptions = ArgumentsCamelCase<GlobalOptionsArgs>;

type ArgsOf<T> = T extends Argv<infer V> ? V : never;

export const applyGlobalOptions = <T>(yargs: Argv<T>) =>
  yargs.option("pack-file", {
    type: "string",
    describe: "Pack file location",
    default: "modpack.toml",
  });
