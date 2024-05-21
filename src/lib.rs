use std::{error, fmt};

/// Type for the result of a command
pub type CommandStringResult = String;

/// Trait designed to be implemented by every addon commands
pub trait MyToolsAddonCommand {
    /// Function to execute the command
    fn execute(&self) -> Result<CommandStringResult, MyToolsError>;

    /// Function to get the input of the command
    fn get_command_input(&self) -> String;

    /// Function to get the help message of the command
    fn get_command_help(&self) -> String;
}

/// Trait designed to be implemented by every addons
pub trait MyToolsAddon {
    /// Function to get the help message of the addon
    fn get_help(&self) -> String {
        // Get addon's keyword
        let keyword = self.get_keyword();

        // Get the list of commands
        let commands = self.get_list_commands();

        // Generate the width of the space separator based on maximum command input length and keyword length
        let width_separator =
            "my_tools".len() // Tool name length
            + keyword.len() // Addon's keyword length
            + commands.iter().map(|cmd| cmd.get_command_input().len()).max().unwrap_or(0) // Max
        // command input length
            + 9;

        // Get the commands input and help message
        let commands: Vec<String> = commands.iter().map(|cmd| {
            format!(
                "{: <width_separator$}{}",
                format!("'my_tools {} {}'", keyword, cmd.get_command_input()),
                cmd.get_command_help(),
                width_separator = width_separator,
            )
        }).collect::<Vec<String>>();

        // Generate the help message
        format!(r#"
=== Addon: {keyword} ===

Usage: {keyword} <COMMAND>

Commands:
  {commands}
"#,
                keyword = keyword,
                commands = commands.join("\n  ")
        )
    }

    /// Function to display the help message if the arguments contains "--help" or "-h"
    fn call_help(&self, args: &Vec<&str>) {
        if args.len() == 1 && (args[0] == "--help" || args[0] == "-h") {
            eprintln!("{}", self.get_help());
            std::process::exit(0); // Exit with success
        }
    }

    // Functions to implement

    /// Function to get the keyword that should be used by the user to call the addon
    fn get_keyword(&self) -> &'static str;

    /// Function to list every commands available
    fn get_list_commands(&self) -> Vec<Box<dyn MyToolsAddonCommand>>;

    /// Function to parse the arguments and return a MyToolsAddonCommand
    fn parse(&self, args: &[String]) -> Result<Box<dyn MyToolsAddonCommand>, MyToolsError>;

}

/// Error type for the addon
#[derive(Debug)]
pub enum MyToolsError {
    /// Error when the addon is not recognized
    AddonNotFound(String),
    /// Error when the command is not well formatted
    InvalidCommand(String),
    /// Error while executing command
    ExecutionCommandError(String)
}

impl fmt::Display for MyToolsError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        let description = match self {
            MyToolsError::AddonNotFound(s) => format!("Unknown addon: {}", s),
            MyToolsError::InvalidCommand(s) => format!("Invalid command: {}", s),
            MyToolsError::ExecutionCommandError(s) => format!("Error while execution command: {}", s),
        };
        f.write_str(&description)
    }
}

impl error::Error for MyToolsError {}
