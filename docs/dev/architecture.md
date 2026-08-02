## Architecture
The project will be implemented in the following layers in order to keep our project organized. A benefit to this structure is that cli logic and business logic should stay separate.

## Context Structs

### CommandContext Struct
The CommandContext struct as as a dependency container for a command. It contains args for the sub command, global args, and any **adapters** it may need. An example of this struct looks like this
```rust
struct CommandContext<CommandArgs, Ui: UiAdapter, Fs : FsAdapter> {
    global_args: GlobalArgs,
    args: CommandArgs,
    ui: Ui,
    fs: Fs
}
```
adapters can be swapped out during testing.

### WorkflowContext Struct
The WorkflowContext struct is similar to the CommandContext but without the Ui Adapter
```rust
struct WorkflowContext<WorkflowArgs, Fs : FsAdapter> {
    global_args: GlobalArgs,
    args: WorkflowArgs,
    fs: Fs
}
```

## Layers

### Root

At the root layer specific global options are processed to determine global functionality such as coloring and output format. **Adapters** are constructed then the subcommand is then resolved and the **dispatcher** layer is called with the **CommandContext**.

### Dispatcher layer

At this layer arguments and flags are processed and the user is prompted for any more information needed. Once input has been processed parameters are constructed and the workflow layer is called. **WorkflowContext** is passed with a new argument type.

### Workflow Layer

At this layer the business logic lives. If an api call or fs call needs to be made an adapter should be used but this logic is solely business logic.

### Structure

```
src/
├── main.rs         # Entrypoint: Clap parsing, definition of global flags, error handling, parsing of global flags, context definition, and adapter creation.
├── commands/       # Command dispatchers
├── workflows/      # Core logic 
└── adapters/       # I/O boundaries encapsulating external actions
    ├── api.rs      # Remote API integrations
    ├── ui.rs       # UI presentation layout (cliclack wrappers)
    └── fs.rs       # Extracted trait interfaces for Mocking filesystem access
```