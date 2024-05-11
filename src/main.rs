use std::collections::HashMap;

use my_tools::MyToolsError;
use my_tools::MyToolsAddon;
use my_tools::CommandStringResult;

// Import the addon modules
mod addons {
    // Step 1 : Define the addon module
    pub mod hello_world;
}

// Step 2 : Import the addon
use addons::hello_world::HelloWorldAddon;

/// Function to call the right addon depending on first argument
fn call_addon(args: Vec<String>, addons: &Vec<Box<dyn MyToolsAddon>>) -> Result<CommandStringResult, MyToolsError> {
    // Get the addon to run
    let addon_to_run = &args[1];

    // HashMap to store keyword and addon
    let addons: HashMap<&str, &Box<dyn MyToolsAddon>> = addons
        .iter()
        .map(|addon| (addon.get_keyword(), addon))
        .collect();

    // Call the right addon, or return an error if not found
    addons.get(addon_to_run.as_str())
        .ok_or(MyToolsError::AddonNotFound(format!("Addon '{}' not found", addon_to_run)))?
        .parse(&args[2..])?
        .execute()
}


/// Function to print the usage of the program
fn print_usage(enabled_addons: &Vec<Box<dyn MyToolsAddon>>) {
    // Print the usage of the program
    eprintln!("Usage: my_tools <addon> [args]\n");
    eprintln!("Available addons:");
    for addon in enabled_addons {
        eprintln!("  - {}", addon.get_keyword());
    }
    eprintln!("\nUse 'my_tools <addon> --help' to get more information about an addon.");
}

/// Function to check if keywords are unique
fn check_keyword_uniqueness(addons: &Vec<Box<dyn MyToolsAddon>>) {
    let mut keywords: Vec<&str> = Vec::new();
    keywords.push("help"); // Reserved keyword

    for addon in addons {
        let keyword = addon.get_keyword();
        if keywords.contains(&keyword) {
            eprintln!("Error: addon keyword '{}' is not unique", keyword);
            std::process::exit(1); // Exit with error code 1, keyword not unique
        }
        keywords.push(keyword);
    }
}

/// Main function
fn main() {
    // Step 3 : Add the addon to the list of enabled addons
    // List of addons
    let enabled_addons: Vec<Box<dyn MyToolsAddon>> = vec![
        Box::new(HelloWorldAddon),
    ];

    // Check if keywords are unique
    check_keyword_uniqueness(&enabled_addons);

    // Get the arguments passed to the program
    let args: Vec<String> = std::env::args().collect::<Vec<String>>();

    // Test arguments length
    if args.len() < 2 {
        print_usage(&enabled_addons);
        std::process::exit(1); // Exit with error code 1, not enough arguments
    }

    // Call the right addon
    match call_addon(args, &enabled_addons) {
        Ok(res) => println!("{}", res),
        Err(e) => {
            eprintln!("/!\\ {}\n", e);
            print_usage(&enabled_addons);
            std::process::exit(1); // Exit with error code 1, addon not found
        }
    }
}
