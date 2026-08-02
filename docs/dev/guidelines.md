Bonsai should follow the guidelines provided by the folks over at [clig.dev](https://clig.dev).
Some of the guidlines that stick out to me that I want to emphasize in this project include.
  
## Human-first Design
Bonsai should assume that this project will be used primarily by humans, so we should be developing for them first and then machines second.

### Success
When a command succeeds Bonsai should print **something**, and exit with exit code 0. At no point should we not print anything on success.

### Errors

#### User-facing Errors
When a command fails we should exit with non-zero exit code and print enough. This means that we should have a concise error message and if we have a hint for what the user should do to resolve this error includde it. miette can help us with this by making sure we include an error message and possible hint.

User-facing errors should not include error codes and instead be something that is helpful to the user.
#### Internal Errors
Internal errors **should** include error codes and if an internal error ever surfaces to the user we should include this error code and a way to submit an issue on github.

Ideally an internal error shouldn't ever surface to the user but if it does happen we should plan for it.

## Messaging

We don't need to treat output as log output. Instead we should only be outputting meaningful messages. Messages primarily should be used when changing the state of the project or crossing boundaries. For more information see [Saying (just) enough](https://clig.dev/#saying-just-enough) and [Output](https://clig.dev/#output)

## Subcommands

[Subcommands](https://clig.dev/#subcommands) is worth a read. A decision that was made for this project is we should be following the `noun verb` standard.

## IO
### Colors
Colors make things easy to read. They should be disabled if any of the following is true:
- output is not a TTY
- env:NO_COLOR is set
- env:BONSAI_NO_COLOR is set
- flag:no-color is set

### Interactivity
Prompts make it easy for the user to interact. They should be disabled if any of the following is true:
- output is not a tty
- env:NO_INPUT is set
- env:BONSAI_NO_INPUT is set
- flag:no-input is set

Additionally the following flags have specific functionality related to Interactivity
| flag | function                   |
| ---- | -------------------------- |
| yes  | answer yes to all confirms |
| no   | answer no to all confirms  |


### Quiet Mode
Quiet mode is used for displaying less output. When quiet mode is enabled Bonsai should not print anything useful only for humans. Quiet mode is true if any of the following is true:
- flag:quiet is set

### Output Format

#### Plain
Plain format should not format the data with colors or any way that isn't one record of data per row. This is useful for tools like sed or awk. This should be enabled if any of the following is true:
- flag:plain

#### JSON
Json format is outputting the data strictly in `json` format this is useful for jq and other tools. This should be enabled if any of the following is true
- flag:json