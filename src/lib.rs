use std::{error, fmt};

/// Type for the result of a command
pub type CommandStringResult = String;

/// Trait designed to be implemented by every addon commands
pub trait MyToolsAddonCommand {
    /// Function to execute the command
    fn execute(&self) -> Result<CommandStringResult, MyToolsError>;
}

/// Trait designed to be implemented by every addons
pub trait MyToolsAddon { 
    /// Function to get the keyword that should be use by the user to call the addon
    fn get_keyword() -> &'static str;

    /// Function to parse the arguments and return a MyToolsAddonCommand
    fn parse(args: &[String]) -> Result<Box<dyn MyToolsAddonCommand>, MyToolsError>;

    /// Function to list every commands available
    fn list_commands() -> Vec<String>;
}

/// Error type for the addon
#[derive( Debug)]
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

