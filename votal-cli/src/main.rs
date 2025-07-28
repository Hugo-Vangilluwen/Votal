// use std::env;
use std::process;

use clap::Parser;

use libvotal::voting_system::find_voting_system;
use votal_cli::read_vote;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The name of the used voting system among plurality
    voting_system: String,

    /// List of choices
    choices: Vec<String>,
}

fn main() {
    // let (config_server, vote) = ConfigServer::build(env::args()).unwrap_or_else(|err| {
    //     eprintln!("Problem parsing arguments: {err}");
    //     process::exit(1);
    // });

    let cli = Cli::parse();

    if cli.choices.len() <= 1 {
        eprintln!("There is not enough choice.");
        process::exit(1);
    }

    let mut vote = find_voting_system(&cli.voting_system).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    })(cli.choices.into_iter());

    read_vote(&mut vote);

    println!("The winner is {}", vote.result());
}
