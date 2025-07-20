use std::collections::HashMap;

/// Describe the ballot form
pub enum BallotForm {
    Uninominal,
}

// Type alias for a ballot box
pub type Ballots = HashMap<String, i32>;
// Type alias for algorithm finding the result of the election from all ballots
pub type ResultAlgorithm = Box<dyn Fn(&Ballots) -> Option<String>>;

/// Used to describe election
pub struct VotingSystem {
    /// The name of the voting system
    name: String,
    /// The ballot form of the voting system
    ballot_form: BallotForm,
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
    pub fn new(
        name: &str,
        ballot_form: BallotForm,
        choices: Vec<String>,
        result_algorithm: ResultAlgorithm,
    ) -> Self {
        let mut choices_hashmap: HashMap<String, i32> = HashMap::new();

        choices.into_iter().map(|s| String::from(s)).for_each(|c| {
            choices_hashmap.insert(c, 0);
        });

        VotingSystem {
            name: String::from(name),
            ballot_form,
            choices: choices_hashmap,
            result_algorithm,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    /// Just vote
    pub fn vote(&mut self, ballot: String) -> Result<(), &'static str> {
        match self.ballot_form {
            BallotForm::Uninominal => {
                self.choices
                    .get(&ballot)
                    .ok_or("Invalid ballot, unknown candidate {ballot}")?;
                self.choices.entry(ballot).and_modify(|count| *count += 1);
                Ok(())
            }
        }
    }

    /// Calculate the election's result
    pub fn result(&self) -> String {
        (self.result_algorithm)(&self.choices).expect("There is no winnner")
    }
}
