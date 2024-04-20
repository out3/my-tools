# Changelog

This file documents all significant changes to this project.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2024-04-20

### Added
- Defined `MyToolsAddon` trait for addons (get_keyword, parse and list_commands).
- Defined `MyToolsAddonCommand` trait for addon commands (execute).
- Defined `MyToolsError` enum for addon errors.
- Defined `CommandStringResult` type alias for command results.

- Implemented `call_addon` function in `main.rs` to call the right addon depending on the first argument.
- Implemented `main` function in `main.rs` to handle the arguments passed to the program.

- Implemented `HelloWorldAddon` structure in `hello_world.rs`.
- Implemented `HelloWorldCommand` in `hello_world.rs` to print "Hello, world!".
- Implemented `HelloInputCommand` in `hello_world.rs` to print "Hello, {name}!".
- Implemented tests for `HelloWorldAddon` in `hello_world.rs`.
 