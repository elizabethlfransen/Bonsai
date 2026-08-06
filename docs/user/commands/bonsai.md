## bonsai  <!-- {docsify-ignore} -->
### Usage
```shell
bonsai [OPTIONS] <COMMAND>
```
### Description
A complete minecraft modpack building cli tool

### Available Commands
* [bonsai init](/user/commands/bonsai/init.md)
* [bonsai completions](/user/commands/bonsai/completions.md)
* [bonsai generate-man](/user/commands/bonsai/generate-man.md)
<div id="commandOptions">

### Global Options
- `--no-color`
  - Disable color in the terminal
- `--force-color`
  - Enables color in the terminal
- `--no-input`
  - Disabled interactivity
- `--json`
  - Formats output as json, implies no-input
- `--plain`
  - Formats output without color or special formatting, implies no-input
- `--quiet`
  - Will not output non-essential output and simplify errors
- `-y`, `--yes`
  - answer yes to all confirms
- `-n`, `--no`
  - answer no to all confirms

</div>
### Examples
#### Initialize a project

```shell
bonsai init
```

#### Add a mod

```shell
bonsai mod add test
```

