// @deno-types="npm:@types/yargs@^17"
import yargs from "yargs";
import init from "@/bin/init.ts";
import add from "@/bin/add.ts";

await yargs(Deno.args)
  // initial configuration
  .scriptName("bonsai")
  .usage("$0 <command> [options]")
  .demandCommand(1)
  // utility commands
  .help()
  .completion()
  .version()
  // global options
  .option('pack-file', {
    type: 'string',
    default: 'modpack.toml'
  })
  .command('$0', 'test', y => y, (argv) => console.log(argv.packFile))
  // commands
  .command(init)
  .command(add)
  // parse
  .parseAsync();
