use std::env;
use std::process;

use votal_cli::ConfigServer;

fn main() {
    let (config_server, vote) = ConfigServer::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("{}", config_server.name);
    println!("{}", vote.get_name())
}
