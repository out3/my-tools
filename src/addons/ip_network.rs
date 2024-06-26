use my_tools::*;
use std::net::Ipv4Addr;
use ipnetwork::Ipv4Network;

// Command to get the IP address
struct GetIpAddressCommand {
    ip_object: Ipv4Network,
}

impl MyToolsAddonCommand for GetIpAddressCommand {
    fn execute(&self) -> Result<CommandResult, MyToolsError> {
        Ok(format!("{}", self.ip_object.ip()))
    }

    fn get_command_input() -> CommandInputs {
        vec![
            "get address <ip/cidr>".to_string(),
            "get address <ip/mask>".to_string(),
            "get address <ip> <cidr>".to_string(),
            "get address <ip> <mask>".to_string(),
        ]
    }

    fn get_command_help() -> CommandHelp {
        "Get the IP address for a given IP object".to_string()
    }
}

// Command to get the netmask address
struct GetIpNetmaskCommand {
    ip_object: Ipv4Network,
}

impl MyToolsAddonCommand for GetIpNetmaskCommand {
    fn execute(&self) -> Result<CommandResult, MyToolsError> {
        Ok(format!("{}", self.ip_object.mask()))
    }

    fn get_command_input() -> CommandInputs {
        vec![
            "get netmask <ip/cidr>".to_string(),
            "get netmask <ip/mask>".to_string(),
            "get netmask <ip> <cidr>".to_string(),
            "get netmask <ip> <mask>".to_string(),
        ]
    }

    fn get_command_help() -> CommandHelp {
        "Get the IP netmask for a given IP object".to_string()
    }
}

// Command to get the network address
struct GetIpNetworkCommand {
    ip_object: Ipv4Network,
}

impl MyToolsAddonCommand for GetIpNetworkCommand {
    fn execute(&self) -> Result<CommandResult, MyToolsError> {
        Ok(format!("{}", self.ip_object.network()))
    }

    fn get_command_input() -> CommandInputs {
        vec![
            "get network <ip/cidr>".to_string(),
            "get network <ip/mask>".to_string(),
            "get network <ip> <cidr>".to_string(),
            "get network <ip> <mask>".to_string(),
        ]
    }

    fn get_command_help() -> CommandHelp {
        "Get the IP network address for a given IP object".to_string()
    }
}
// Addon structure
pub struct IpNetworkAddon;

impl MyToolsAddon for IpNetworkAddon {
    fn get_keyword(&self) -> &'static str {
        "ipnet"
    }

    fn parse(&self, args: &[String]) -> Result<Box<dyn MyToolsAddonCommand>, MyToolsError> {
        // Convert &[String] to &[&str]
        let args: Vec<&str> = args
            .iter()
            .map(|s| s.as_str())
            .collect();

        // Get the help message if args correspond to "--help" or "-h"
        self.call_help(&args);

        // Parse the arguments and return the corresponding command
        match args[..] {
            // GetIpAddress
            ["get", "address", arg1] => {
                let ip_object = arg_to_ipv4network(arg1, None)?;
                Ok(Box::new(GetIpAddressCommand { ip_object }))
            },
            ["get", "address", arg1, arg2] => {
                let ip_object = arg_to_ipv4network(arg1, Some(arg2))?;
                Ok(Box::new(GetIpAddressCommand { ip_object }))
            },
            // GetIpNetmask
            ["get", "netmask", arg1] => {
                let ip_object = arg_to_ipv4network(arg1, None)?;
                Ok(Box::new(GetIpNetmaskCommand { ip_object }))
            },
            ["get", "netmask", arg1, arg2] => {
                let ip_object = arg_to_ipv4network(arg1, Some(arg2))?;
                Ok(Box::new(GetIpNetmaskCommand { ip_object }))
            },
            // GetIpNetwork
            ["get", "network", arg1] => {
                let ip_object = arg_to_ipv4network(arg1, None)?;
                Ok(Box::new(GetIpNetworkCommand { ip_object }))
            },
            ["get", "network", arg1, arg2] => {
                let ip_object = arg_to_ipv4network(arg1, Some(arg2))?;
                Ok(Box::new(GetIpNetworkCommand { ip_object }))

            },
            _ => Err(MyToolsError::InvalidCommand(format!("Invalid command: {}\n", args.join(" "))))
        }
    }


    fn get_list_commands(&self) -> Vec<CommandInputsHelp> {
        vec![
            CommandInputsHelp {
                inputs_msg: GetIpAddressCommand::get_command_input(),
                help_msg: GetIpAddressCommand::get_command_help()
            },
            CommandInputsHelp {
                inputs_msg: GetIpNetmaskCommand::get_command_input(),
                help_msg: GetIpNetmaskCommand::get_command_help()
            },
            CommandInputsHelp {
                inputs_msg: GetIpNetworkCommand::get_command_input(),
                help_msg: GetIpNetworkCommand::get_command_help()
            },
        ]
    }
}


/// Function to parse arguments to a IPv4Network object
fn arg_to_ipv4network(arg1: &str, arg2: Option<&str>) -> Result<Ipv4Network, MyToolsError> {
    match arg2 {
        // Handling 2 arguments
        Some(cidr_netmask) => {
            // Try to parse arg1 as an Ipv4Addr, if not, return ParseCommandError
            return parse_args(arg1, cidr_netmask)
        },
        // Handling 1 argument
        None => {
            // Check if arg1 contains the '/' separator
            if !arg1.contains("/") {
                return Err(MyToolsError::ParseCommandError("Separator '/' is missing".to_string()))
            };

            // Split the argument into ip and cidr/netmask
            let (ip, cidr_netmask) = arg1.split_once("/").unwrap();

            return parse_args(ip, cidr_netmask)
        }
    }

    // DRY
    fn parse_args(arg1: &str, arg2: &str) -> Result<Ipv4Network, MyToolsError> {
        if let Ok(ip) = arg1.parse::<Ipv4Addr>() {
            // Check if cidr_netmask is not empty
            if arg2.is_empty() {
                return Err(MyToolsError::ParseCommandError("Argument is missing a CIDR or a netmask".to_string()))
            }

            // Try to parse arg2 as a cidr
            if let Ok(cidr) = arg2.parse::<u8>() {
                if cidr <= 32 {
                    return Ok(Ipv4Network::new(ip, cidr).unwrap())
                }
            } else {
                // Try to parse arg2 as a netmask
                if let Ok(netmask) = arg2.parse::<Ipv4Addr>() {
                    if is_netmask(&netmask) {
                        return Ok(Ipv4Network::with_netmask(ip, netmask).unwrap())
                    }
                }
            }

            // Return Err in case arg2 is neither a valid CIDR nor a valid netmask
            Err(MyToolsError::ParseCommandError(format!("Argument '{}' is neither a valid CIDR nor a netmask", arg2)))
        } else {
            Err(MyToolsError::ParseCommandError(format!("Invalid IP address: '{}'", arg1)))
        }
    }

    fn is_netmask(addr: &Ipv4Addr) -> bool {
        let addr_bit = addr.octets()
            .iter()
            .fold(0u32, |acc, &octet| acc << 8 | octet as u32);

        let mut found_zero = false;
        for i in 0..32 {
            if addr_bit & (1 << (31 - i)) == 0 {
                found_zero = true;
            } else if found_zero {
                return false;
            }
        }
        true
    }
}

// Tests for arg_to_ipv4network
// Tests with 1 argument
#[test]
fn arg_to_ipv4network_test_1_arg_ip_nok() {
    let arg1 = "this is not an ip/8";
    let arg_object = arg_to_ipv4network(arg1, None);
    assert_eq!(arg_object.unwrap_err(), MyToolsError::ParseCommandError(format!("Invalid IP address: '{}'", arg1.split_once("/").unwrap().0)))
}
#[test]
fn arg_to_ipv4network_test_1_arg_no_separator() {
    let arg1 = "127.0.0.1-8";
    let arg_object = arg_to_ipv4network(arg1, None);
    assert_eq!(arg_object.unwrap_err(), MyToolsError::ParseCommandError("Separator '/' is missing".to_string()))
}

#[test]
fn arg_to_ipv4network_test_1_arg_ip_ok_cidr_or_netmask_missing_1() {
    let arg1 = "127.0.0.1";
    let arg_object = arg_to_ipv4network(arg1, None);
    assert_eq!(arg_object.unwrap_err(), MyToolsError::ParseCommandError("Separator '/' is missing".to_string()))
}

#[test]
fn arg_to_ipv4network_test_1_arg_ip_ok_cidr_or_netmask_missing_2() {
    let arg1 = "127.0.0.1/";
    let arg_object = arg_to_ipv4network(arg1, None);
    assert_eq!(arg_object.unwrap_err(), MyToolsError::ParseCommandError("Argument is missing a CIDR or a netmask".to_string()))
}

#[test]
fn arg_to_ipv4network_test_1_arg_ip_ok_cidr_ok() {
    let arg1 = "127.0.0.1/24";
    let arg_object = arg_to_ipv4network(arg1, None);
    assert_eq!(arg_object.unwrap(), Ipv4Network::new(Ipv4Addr::new(127, 0, 0, 1), 24).unwrap())
}

#[test]
fn arg_to_ipv4network_test_1_arg_ip_ok_cidr_nok() {
    let arg1 = "127.0.0.1/33";
    let arg_object = arg_to_ipv4network(arg1, None);
    assert_eq!(arg_object.unwrap_err(), MyToolsError::ParseCommandError(format!("Argument '{}' is neither a valid CIDR nor a netmask", arg1.split_once("/").unwrap().1)))
}

#[test]
fn arg_to_ipv4network_test_1_arg_ip_ok_netmask_ok() {
    let arg1 = "127.0.0.1/255.0.0.0";
    let arg_object = arg_to_ipv4network(arg1, None);
    assert_eq!(arg_object.unwrap(), Ipv4Network::with_netmask(Ipv4Addr::new(127, 0, 0, 1), Ipv4Addr::new(255, 0, 0, 0)).unwrap())
}

#[test]
fn arg_to_ipv4network_test_1_arg_ip_ok_netmask_nok() {
    let arg1 = "127.0.0.1/255.255.255.256";
    let arg_object = arg_to_ipv4network(arg1, None);
    assert_eq!(arg_object.unwrap_err(), MyToolsError::ParseCommandError(format!("Argument '{}' is neither a valid CIDR nor a netmask", arg1.split_once("/").unwrap().1)))
}

// Tests with 2 arguments
#[test]
fn arg_to_ipv4network_test_2_args_ip_nok_1() {
    let arg1 = "this is not an ip";
    let arg2 = "8";
    let arg_object = arg_to_ipv4network(arg1, Some(arg2));
    assert_eq!(arg_object.unwrap_err(), MyToolsError::ParseCommandError(format!("Invalid IP address: '{}'", arg1)))
}

#[test]
fn arg_to_ipv4network_test_2_args_ip_nok_2() {
    let arg1 = "127.0.0.256";
    let arg2 = "8";
    let arg_object = arg_to_ipv4network(arg1, Some(arg2));
    assert_eq!(arg_object.unwrap_err(), MyToolsError::ParseCommandError(format!("Invalid IP address: '{}'", arg1)))
}

#[test]
fn arg_to_ipv4network_test_2_arg_ip_ok_cidr_or_netmask_missing() {
    let arg1 = "127.0.0.1";
    let arg2 = "";
    let arg_object = arg_to_ipv4network(arg1, Some(arg2));
    assert_eq!(arg_object.unwrap_err(), MyToolsError::ParseCommandError("Argument is missing a CIDR or a netmask".to_string()))
}

#[test]
fn arg_to_ipv4network_test_2_args_ip_ok_cidr_ok() {
    let arg_object = arg_to_ipv4network("127.0.0.1", Some("8")).unwrap();
    let ip_object = Ipv4Network::new(Ipv4Addr::new(127, 0, 0, 1), 8).unwrap();
    assert_eq!(arg_object, ip_object)
}

#[test]
fn arg_to_ipv4network_test_2_args_ip_ok_cidr_nok() {
    let arg2 = "33";
    let arg_object = arg_to_ipv4network("127.0.0.1", Some(arg2));
    assert_eq!(arg_object.unwrap_err(), MyToolsError::ParseCommandError(format!("Argument '{}' is neither a valid CIDR nor a netmask", arg2)))
}

#[test]
fn arg_to_ipv4network_test_2_args_ip_ok_netmask_ok() {
    let arg_object = arg_to_ipv4network("127.0.0.1", Some("255.255.255.0")).unwrap();
    let ip_object = Ipv4Network::with_netmask(Ipv4Addr::new(127, 0, 0, 1), Ipv4Addr::new(255, 255, 255, 0)).unwrap();
    assert_eq!(arg_object, ip_object)
}

#[test]
fn arg_to_ipv4network_test_2_args_ip_ok_netmask_nok_1() {
    let arg2 = "255.255.255.256";
    let arg_object = arg_to_ipv4network("127.0.0.1", Some(arg2));
    assert_eq!(arg_object.unwrap_err(), MyToolsError::ParseCommandError(format!("Argument '{}' is neither a valid CIDR nor a netmask", arg2)))
}

#[test]
fn arg_to_ipv4network_test_2_args_ip_ok_netmask_nok_2() {
    let arg2 = "255.255.0.128";
    let arg_object = arg_to_ipv4network("127.0.0.1", Some(arg2));
    assert_eq!(arg_object.unwrap_err(), MyToolsError::ParseCommandError(format!("Argument '{}' is neither a valid CIDR nor a netmask", arg2)))
}
