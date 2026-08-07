## bonsai generate-man  <!-- {docsify-ignore} -->
### Usage
```shell
bonsai generate-man [OPTIONS] [OUT]
```
### Description
Generate man pages. If the directory already exists this will fail. If you want to update your man pages use `--force`

<div id="commandOptions">

### Positional Arguments
- `OUT`
  - output directory to generate man args
### Options
- `-f`, `--force`
  - Forces man pages to generate even if they are already present
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
#### Generate man pages and put them in /usr/local/share/man/man1

```shell
bonsai generate-man
```

#### Generate man pages and store them locally

```shell
bonsai generate-man ./man
```

