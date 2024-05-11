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
    // Get the addon to run
    let addon_to_run = &args[1];

    match addon_to_run.as_str() {
        "hello" => {
            HelloWorldAddon::parse(&args[2..])?
                .execute()
        }
        _ => Err(MyToolsError::AddonNotFound(format!("Addon '{}' not found", addon_to_run)))
    }
}


/// Function to print the usage of the program
fn print_usage() {
    // List of addons's
    let addons_keyword = vec![
        HelloWorldAddon::get_keyword,
    ];

    // Print the usage of the program
    eprintln!("Usage: my_tools <addon> [args]\n");
    eprintln!("Available addons:");
    for keyword in addons_keyword {
        eprintln!("  - {}", keyword());
    }
    eprintln!("\nUse 'my_tools <addon> --help' to get more information about an addon.");
}


/// Main function
fn main() {
    // Get the arguments passed to the program
    let args: Vec<String> = std::env::args().collect::<Vec<String>>();

    // Test arguments length
    if args.len() < 2 {
        print_usage();
        return;
    }

    // Call the right addon
    match call_addon(args) {
        Ok(res) => println!("{}", res),
        Err(e) => eprintln!("{}", e)
    }
}
