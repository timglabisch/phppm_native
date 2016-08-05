extern crate clap;
extern crate mio;
mod config;
mod output;
mod process_manager;
mod controller;
use clap::{Arg, App};
use config::Config;
use process_manager::ProcessManager;
use output::{OutputConsole, OutputTrait};
use controller::ControllerHandler;

const SERVER: ::mio::Token = ::mio::Token(0);

fn main() {

    let matches = App::new("PHPPM (Native)")
        .version("0.1")
        .author("Tim Glabisch. <php-pm-native@tim.ainfach.de>")
        .about("Native PHPPM Server")
        .arg(Arg::with_name("working-directory")
            .value_name("working-directory")
            .help("the root of your application (default ./)")
            .takes_value(true)
            .default_value("./")
            .index(1)
        )
        .arg(Arg::with_name("host")
            .help("Load-Balancer host. Default is 127.0.0.1")
            .long("host")
            .default_value("127.0.0.1")
        )
        .arg(Arg::with_name("port")
            .help("Load-Balancer port. Default is 8080")
            .long("port")
        )
        .arg(Arg::with_name("workers")
            .help("Number of Workers. Default is 8.")
            .long("workers")
            .default_value("8")
        )
        .get_matches();

    let config = Config {
        working_directory: matches.value_of("working-directory").unwrap_or("./").to_string(),
        host: matches.value_of("host").unwrap_or("127.0.0.1").to_string(),
        port: matches.value_of("port").unwrap_or("8080").parse::<usize>().expect("port must be a positive number"),
        workers: matches.value_of("workers").unwrap_or("8").parse::<usize>().expect("workers must be a positive number")
    };

    println!("Starting PHPPHM (Native)\n");
    println!("Configuration: {:#?}\n", config);

    let output = OutputConsole::new();

    let process_manager = ProcessManager::new(&output, config.port, config.host, config.workers);
    process_manager.run();
}
