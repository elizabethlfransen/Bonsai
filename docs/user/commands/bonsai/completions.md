## bonsai completions  <!-- {docsify-ignore} -->
### Usage
```shell
bonsai completions [SHELL]
```
### Description
Generate shell completions.

Typically you would add the follow line in your .bashrc or equivalent file:

```bash eval "$(bonsai completions)" ```

<div id="commandOptions">

### Positional Arguments
- `SHELL`
  - The target shell
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
#### Generate completions, detecting shell automatically

```shell
bonsai completions
```

#### Genenerate completions for a specifc shell

```shell
bonsai completions --shell zsh
```

