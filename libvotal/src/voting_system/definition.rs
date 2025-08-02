use std::collections::HashMap;
use std::error::Error;
use std::fmt;

/// Describe the ballot form
pub enum BallotForm {
    Uninominal,
}

/// Type for a ballot box
pub enum Ballots {
    Uninominal(HashMap<String, i32>),
}

impl Ballots {
    fn new(ballot_form: BallotForm, choices: impl Iterator<Item = String>) -> Self {
        match ballot_form {
            BallotForm::Uninominal => {
                let mut choices_hashmap: HashMap<String, i32> = HashMap::new();

                choices.map(|s| String::from(s)).for_each(|c| {
                    choices_hashmap.insert(c, 0);
                });
                Ballots::Uninominal(choices_hashmap)
            }
        }
    }

    fn choices(&self) -> impl Iterator<Item = &String> {
        match &self {
            Ballots::Uninominal(c) => c.keys(),
        }
    }

    fn ballot_form(&self) -> BallotForm {
        match self {
            Ballots::Uninominal(_) => BallotForm::Uninominal,
        }
    }
}

/// Type for algorithm finding the result of the election from all ballots
pub type ResultAlgorithm = Box<dyn Fn(&Ballots) -> Option<String>>;

/// Used to describe election
pub struct VotingSystem {
    /// The name of the voting system
    name: String,
    // The ballot form of the voting system
    // ballot_form: BallotForm,
    /// All ballots of the voting system
    ballot_box: Ballots,
    /// Calculate the election's result
    result_algorithm: ResultAlgorithm,
    /// Total number of ballots
    count: usize,
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
            ballot_box: Ballots::new(ballot_form, choices),
            result_algorithm,
            count: 0,
        }
    }

    /// Get the name of the voting system
    pub fn get_name(&self) -> &str {
        &self.name[..]
    }

    /// Get all choices
    pub fn get_choices(&self) -> impl Iterator<Item = &String> {
        self.ballot_box.choices()
    }

    /// Get the ballot form
    pub fn get_ballot_form(&self) -> BallotForm {
        self.ballot_box.ballot_form()
    }

    /// Get the total number of ballots
    pub fn get_count(&self) -> usize {
        self.count
    }

    /// Just vote
    pub fn vote(&mut self, ballot: &str) -> Result<(), InvalidBallot> {
        match &mut self.ballot_box {
            Ballots::Uninominal(c) => {
                c.get(ballot)
                    .ok_or(InvalidBallot(format!("unknown candidate {}", ballot)))?;
                c.entry(String::from(ballot))
                    .and_modify(|count| *count += 1);
            }
        }

        self.count += 1;
        Ok(())
    }

    /// Calculate the election's result
    pub fn result(&self) -> String {
        (self.result_algorithm)(&self.ballot_box).expect("There is no winnner")
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic]
    fn invalid_ballot() {
        let mut p = VotingSystem::new(
            "test",
            BallotForm::Uninominal,
            vec![String::from("A")].into_iter(),
            Box::new(|_choices: &Ballots| Some(String::from("A"))),
        );

        p.vote("C").unwrap();
    }
}
