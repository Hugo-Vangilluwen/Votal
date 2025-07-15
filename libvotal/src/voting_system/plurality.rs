use std::collections::HashMap;

use crate::voting_system::definition::*;

/// First-past-the-post voting
pub struct Plurality {
    choices: HashMap<String, i32>,
}

impl Plurality {
    /// Create a new plurality election
    pub fn new(choices: Vec<&str>) -> Self {
        let mut choices_hashmap: HashMap<String, i32> = HashMap::new();

        choices.into_iter().map(|s| String::from(s)).for_each(|c| {
            choices_hashmap.insert(c, 0);
        });

        Plurality { choices: choices_hashmap }
    }
}

impl Voting for Plurality {
    fn get_name(&self) -> String {
        String::from("Plurality")
    }

    fn get_choices(&self) -> impl Iterator<Item = &String> {
        self.choices.keys()
    }

    fn get_ballot_form(&self) -> BallotForm {
        BallotForm::Uninominal
    }

    fn vote(&mut self, ballot: String) -> Result<(), &'static str> {
        self.choices
            .get(&ballot)
            .ok_or("Invalid ballot, unknown candidate {ballot}")?;
        self.choices.entry(ballot).and_modify(|count| *count += 1);
        Ok(())
    }

    fn result<'a>(&'a self) -> &'a str {
        self.choices
            .iter()
            .max_by(|a, b| a.1.cmp(&b.1))
            .map(|(k, _v)| k)
            .expect("There is no winnner")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plurality_voting() {
        let mut p = Plurality::new(vec!["A", "B", "C"]);
        for v in vec!["A", "B", "A", "C"] {
            if let Err(e) = p.vote(String::from(v)) {
                panic!("{e}")
            }
        }

        assert_eq!("A", p.result());
    }
}
