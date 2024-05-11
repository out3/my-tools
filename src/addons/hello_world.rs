use my_tools::MyToolsAddon;
use my_tools::MyToolsAddonCommand;
use my_tools::MyToolsError;
use my_tools::CommandStringResult;

/// Command to print "Hello, world!"
struct HelloWorldCommand {}

impl MyToolsAddonCommand for HelloWorldCommand {
    fn execute(&self) -> Result<CommandStringResult, MyToolsError> {
        Ok("Hello, world!".to_string())
    }
}

/// Command to print "Hello, <name>!"
struct HelloInputCommand {
    name: String,
}

impl MyToolsAddonCommand for HelloInputCommand {
    fn execute(&self) -> Result<CommandStringResult, MyToolsError> {
        Ok(format!("Hello, {}!", self.name))
    }
}


/// HelloWorldAddon structure
pub struct HelloWorldAddon;

impl MyToolsAddon for HelloWorldAddon {
    /// Get the keyword of the addon
    fn get_keyword(&self) -> &'static str {
        "hello"
    }

    /// Parse the arguments and return the corresponding command
    fn parse(&self, args: &[String]) -> Result<Box<dyn MyToolsAddonCommand>, MyToolsError> {
        // Convert &[String] to &[&str]
        let args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();

        // Get the help message if args correspond to "--help" or "-h"
        self.call_help(&args);

        // Parse the arguments and return the corresponding command
        match args[..] {
            [] => Ok(Box::new(HelloWorldCommand {})),
            [name] => Ok(Box::new(HelloInputCommand { name: name.to_string() })),
            _ => Err(MyToolsError::InvalidCommand(format!("Invalid command : {}\n", args.join(" "))))
        }
    }    

    /// List the commands of the addon
    fn list_commands(&self) -> Vec<String> {
        let width = 30;
        vec![
            format!(
                "{: <width$}Print \"Hello, world!\"",
                format!("'my_tools {}'", self.get_keyword()),
            ),
            format!(
                "{: <width$}Print \"Hello, <name>!\"", 
                format!("'my_tools {} <name>'", self.get_keyword()),
            ),
        ]
    }
}

#[test]
fn get_keyword() {
    let keyword = HelloWorldAddon.get_keyword();
    assert_eq!(keyword, "hello");
}

// HelloWorldAddon::parse tests
// "hello" -> HelloWorldCommand - Ok
#[test]
fn command_hello() {
    let args = vec![];
    let cmd = HelloWorldAddon.parse(&args).expect("Failed to parse command");
    assert_eq!(cmd.execute().unwrap(), String::from("Hello, world!"));
}

// "hello world123" -> HelloInputCommand -> Ok
#[test]
fn command_hello_input() {
    let args = vec!["world123".to_string()];
    let cmd = HelloWorldAddon.parse(&args).expect("Failed to parse command");
    assert_eq!(cmd.execute().unwrap(), String::from("Hello, world123!"));
}

// "hello world test" -> InvalidCommand -> Error
#[test]
fn parse_over_args() {
    let args = vec!["world".to_string(), "test".to_string()];
    let cmd = HelloWorldAddon.parse(&args);
    assert!(cmd.is_err());
}
