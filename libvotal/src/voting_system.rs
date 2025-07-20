mod definition;
pub use self::definition::{BallotForm, VotingSystem};

mod plurality;
pub use self::plurality::plurality;

pub fn find_voting_system(name: &str) -> fn(Vec<String>) -> definition::VotingSystem {
    match name {
        self::plurality::NAME => plurality,
        _ => panic!("Unknown voting system {name}"),
    }
}
