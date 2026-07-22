import yargs from "yargs";

yargs(Deno.args)
  .scriptName('bonsai')
  .usage('$0 <command> [options]')
  .command("ping", "pong!", () => console.log("pong!"))
  .demandCommand(1, 'You need at least one command before moving on.')
  .help()
  .parse();
