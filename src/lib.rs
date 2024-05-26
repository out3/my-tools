use std::{error, fmt};

/// Type for the result of a command
pub type CommandResult = String;
/// Type for the input messages of a command
pub type CommandInputs = Vec<String>;
/// Type for the help message of a command
pub type CommandHelp = String;
/// Struct to group command inputs and help message
pub struct CommandInputsHelp {
    pub inputs_msg: CommandInputs,
    pub help_msg: CommandHelp
}

/// Trait designed to be implemented by every addon commands
pub trait MyToolsAddonCommand {
    /// Function to execute the command
    fn execute(&self) -> Result<CommandResult, MyToolsError>;

    /// Function to get the input of the command
    fn get_command_input() -> CommandInputs where Self: Sized;

    /// Function to get the help message of the command
    fn get_command_help() -> CommandHelp where Self: Sized;
}

/// Trait designed to be implemented by every addons
pub trait MyToolsAddon {
    /// Function to get the help message of the addon
    fn get_help(&self) -> String {
        // Get addon's keyword
        let keyword: &str = self.get_keyword();

        // Get the list of commands
        let command_list: Vec<CommandInputsHelp> = self.get_list_commands();

        // Get the commands input and help message
        let commands_text_message: Vec<String> = command_list.iter().map(|cmd| {
            // Generate list of command inputs
            let command_inputs = cmd.inputs_msg
                .iter()
                .map(|input| {
                    format!("\t\tmy_tools {} {}", keyword, input)
                })
                .collect::<Vec<String>>();

            // Generate the whole command message
            format!(
                "\t{}\n{}",
                cmd.help_msg,
                command_inputs.join("\n")
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
            commands = commands_text_message.join("\n")
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
    fn get_list_commands(&self) -> Vec<CommandInputsHelp>;

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
