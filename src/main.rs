use my_tools::MyToolsError;
use my_tools::MyToolsAddon;
use my_tools::CommandStringResult;

// Import the addon modules
mod addons {
    pub mod hello_world;
}

use addons::hello_world::HelloWorldAddon;


/// Function to call the right addon depending on first argument
fn call_addon(args: Vec<String>) -> Result<CommandStringResult, MyToolsError> {
    let addon_to_run = &args[1];
    match addon_to_run.as_str() {
        "hello" => {
            HelloWorldAddon::parse(&args[2..])?
                .execute()
        }
        _ => Err(MyToolsError::AddonNotFound(format!("Addon '{}' not found", addon_to_run)))
    }
}

fn main() {
    // Get the arguments passed to the program
    let args: Vec<String> = std::env::args().collect::<Vec<String>>();

    // Test arguments length
    if args.len() < 2 {
        // TODO : Print the usage of the program
        eprintln!("TODO Usage");
        return;
    }

    match call_addon(args) {
        Ok(res) => println!("{}", res),
        Err(e) => eprintln!("{}", e)
    }
}
