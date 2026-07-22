import { ArgumentsCamelCase, Argv } from "yargs";

export interface GlobalOptionsArgs {
    'pack-file': string
}

export type GlobalOptions = ArgumentsCamelCase<GlobalOptionsArgs>;

export function applyGlobalOptions<T={}>(yargs: Argv<T>): Argv<T & GlobalOptionsArgs> {
    return yargs.option('pack-file', {
        type: 'string',
        describe: 'Pack file location',
        default: 'modpack.toml',
    });
}