use std::error::Error;
use std::fmt;

mod definition;
pub use self::definition::{BallotForm, VotingSystem};

mod plurality;
pub use self::plurality::plurality;

/// Error for unknown voting system
#[derive(Debug)]
pub struct UnknownVotingSystem(String);

impl fmt::Display for UnknownVotingSystem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unknown voting system: {}", self.0)
    }
}

impl Error for UnknownVotingSystem {}

/// Try to find the voting system which is associated to name
/// Return a function to create the voting system if found
/// And return a UnknownVotingSystem error else
pub fn find_voting_system<T>(
    name: &str,
) -> Result<fn(T) -> definition::VotingSystem, UnknownVotingSystem>
where
    T: Iterator<Item = String>,
{
    match name {
        self::plurality::NAME => Ok(plurality),
        _ => Err(UnknownVotingSystem(format!("{}", name))),
    }
}
