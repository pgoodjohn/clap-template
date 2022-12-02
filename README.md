# Clap Template

Skeleton application to build cli tools using [clap](https://clap.rs).

## Adding Commands

To add new commands, copy the `template` folder:

```sh
$ cp -r template/ new_command/
```

Then rename the instances of `Template` in the copied `mod.rs` to your desired command name:

```sh
$ sed -i '' 's/Template/NewCommand/g' ./src/new_command/mod.rs
```

Include `new_command` in `main.rs`:

```rust
// [...]
mod new_command;
// [...]
```

And ensure it is available as a top level command:

```rust
// [...]
enum Commands {
    // [...]
    NewCommand(new_command::Command),
    // [...]
}
// [...]
fn main() {
    // [...]
    match cli.command {
        // [...]
        Some(Commands::NewCommand(command)) => {
            new_command::command(&command);
        },
        //[...]
    }
    // [...]
}
```

You can then try out your new command by running the application and invoking `new_command`:

```sh
$ cargo run -- new_command
```