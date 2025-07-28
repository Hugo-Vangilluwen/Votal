use crate::voting_system::definition::*;

pub const NAME: &str = "plurality";

/// First-past-the-post voting
pub fn plurality(choices: impl Iterator<Item = String>) -> VotingSystem {
    VotingSystem::new(
        NAME,
        BallotForm::Uninominal,
        choices,
        Box::new(|choices: &Ballots| {
            match choices {
                Ballots::Uninominal(c) => c
                    .iter()
                    .max_by(|a, b| a.1.cmp(&b.1))
                    .map(|(k, _v)| k)
                    .cloned(),
                // _ => unimplemented!()
            }
        }),
    )
}

// pub struct Plurality {
//     choices: HashMap<String, i32>,
// }
//
// impl Plurality {
//     /// Create a new plurality election
//     pub fn new(choices: Vec<&str>) -> Self {
//         let mut choices_hashmap: HashMap<String, i32> = HashMap::new();
//
//         choices.into_iter().map(|s| String::from(s)).for_each(|c| {
//             choices_hashmap.insert(c, 0);
//         });
//
//         Plurality { choices: choices_hashmap }
//     }
// }

// impl Voting for Plurality {
//     const NAME: &str = "Plurality";
//     const BALLOT_FORM: BallotForm = BallotForm::Uninominal;
//
//     fn get_choices(&self) -> impl Iterator<Item = &String> {
//         self.choices.keys()
//     }
//
//     fn vote(&mut self, ballot: String) -> Result<(), &'static str> {
//         self.choices
//             .get(&ballot)
//             .ok_or("Invalid ballot, unknown candidate {ballot}")?;
//         self.choices.entry(ballot).and_modify(|count| *count += 1);
//         Ok(())
//     }
//
//     fn result<'a>(&'a self) -> &'a str {
//         self.choices
//             .iter()
//             .max_by(|a, b| a.1.cmp(&b.1))
//             .map(|(k, _v)| k)
//             .expect("There is no winnner")
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plurality_voting() {
        let mut p =
            plurality(vec![String::from("A"), String::from("B"), String::from("C")].into_iter());
        for v in vec!["A", "B", "A", "C"] {
            if let Err(e) = p.vote(String::from(v)) {
                panic!("{e}")
            }
        }

        assert_eq!("A", p.result());
    }
}
