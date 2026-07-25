# Bonsai

A simple to use cli tool for managing multi-format packs

## Installation

### Pre-built Binaries

The latest builds are available via GitHub actions.

#### Stable Releases

<!-- START_STABLE_RELEASE_TABLE -->
| Platform | Download            |
| -------- | ------------------- |
| windows  | _Not yet available_ |
| mac      | _Not yet available_ |
| linux    | _Not yet available_ |
<!-- END_STABLE_RELEASE_TABLE -->

#### Latest Releases

<!-- START_LATEST_RELEASE_TABLE -->
| Platform | Download            |
| -------- | ------------------- |
| windows  | _Not yet available_ |
| mac      | _Not yet available_ |
| linux    | _Not yet available_ |
<!-- END_LATEST_RELEASE_TABLE -->

### Building fromm source

#### Prerequisites

- Deno runtime installed on your system

#### Build Steps

1. Clone this repository to your local machine.
2. Open your terminal in the project root directory.
3. Run the compilation task:

```bash
deno task compile
```

4. The standalone executable will be generated in the root folder:

```bash
./bonsai
```
## Docs
link to docs will go here

## Development

### Packwiz Inspiration
I would like to thank the developers of [Packwiz](https://packwiz.infra.link/). This project was heavily inspired by packwiz but I wanted to take it in another direction, removing individual mod meta files in favor a modlist file and lockfile. A lot of this project will code ported from their project on [GitHub](https://github.com/packwiz/packwiz) licensed under MIT. 

### Testing Locally
To run Bonsai without compiling you can you use the `dev` task. This will execute Bonsai within the `test-project` directory

```bash
mkdir test-project # Create the test directory
cd test-project    # Optionally set your working directory to the new project but it's not required
deno task dev help # Print the help command
```
### Automated Testing
To run automated testing run the `test` sub command. This will create a temp directory and execute commands for testing

```bash
deno test
```
### AI Disclaimer
This project was not built with AI, although may be influenced somewhat by AI. I've used to answer questions because it's faster than search ask overflow, but I do not copy and paste code.