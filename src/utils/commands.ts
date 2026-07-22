import type { ArgumentsCamelCase, Argv, CommandModule } from "yargs";
import { GlobalOptionsArgs } from "@/bin/globalOptions.ts";


export function defineCommand<T=GlobalOptionsArgs, U={}>(cmd: CommandModule<T,U>): CommandModule<T, U> { return cmd; }