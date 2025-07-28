use std::collections::HashMap;
use std::error::Error;
use std::fmt;

/// Describe the ballot form
pub enum BallotForm {
    Uninominal,
}

// Type for a ballot box
pub(crate) enum Ballots {
    Uninominal(HashMap<String, i32>),
}
// Type for algorithm finding the result of the election from all ballots
pub type ResultAlgorithm = Box<dyn Fn(&Ballots) -> Option<String>>;

/// Used to describe election
pub struct VotingSystem {
    /// The name of the voting system
    name: String,
    // The ballot form of the voting system
    // ballot_form: BallotForm,
    /// All ballots of the voting system
    choices: Ballots,
    /// Calculate the election's result
    result_algorithm: ResultAlgorithm,
    // Give all choices
    // fn get_choices(&self) -> impl Iterator<Item = &String>;
    // Just vote
    // fn vote(&mut self, ballot: String) -> Result<(), &'static str>;
    // Calculate the election's result
    // fn result<'a>(&'a self) -> &'a str;
}

impl VotingSystem {
    /// Create a new election
    pub(crate) fn new(
        name: &str,
        ballot_form: BallotForm,
        choices: impl Iterator<Item = String>,
        result_algorithm: ResultAlgorithm,
    ) -> Self {
        VotingSystem {
            name: String::from(name),
            // ballot_form,
            choices: match ballot_form {
                BallotForm::Uninominal => {
                    let mut choices_hashmap: HashMap<String, i32> = HashMap::new();

                    choices.map(|s| String::from(s)).for_each(|c| {
                        choices_hashmap.insert(c, 0);
                    });
                    Ballots::Uninominal(choices_hashmap)
                }
            },
            result_algorithm,
        }
    }

    /// Get the name of the voting system
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    /// Get all choices
    pub fn get_choices(&self) -> impl Iterator<Item = &String> {
        match &self.choices {
            Ballots::Uninominal(c) => c.keys()
        }
    }

    /// Just vote
    pub fn vote(&mut self, ballot: String) -> Result<(), InvalidBallot> {
        match &mut self.choices {
            Ballots::Uninominal(c) => {
                c.get(&ballot)
                    .ok_or(InvalidBallot(format!("unknown candidate {}", ballot)))?;
                c.entry(ballot).and_modify(|count| *count += 1);
                Ok(())
            }
        }
    }

    /// Calculate the election's result
    pub fn result(&self) -> String {
        (self.result_algorithm)(&self.choices).expect("There is no winnner")
    }
}

/// Error  for invalid ballot
#[derive(Debug)]
pub struct InvalidBallot(String);

impl fmt::Display for InvalidBallot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid ballot: {}", self.0)
    }
}

impl Error for InvalidBallot {}
