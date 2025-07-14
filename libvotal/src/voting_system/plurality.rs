use crate::voting_system::definition::*;

pub struct Plurality {
    pub choices: Vec<String>,
}

/// First-past-the-post voting
impl Plurality {
    fn new(choices: impl Iterator<Item = String>) -> Self {
        Plurality {
            choices: choices.collect(),
        }
    }
}

impl Voting for Plurality {
    fn get_name(&self) -> String {
        String::from("Plurality")
    }

    fn get_choices(&self) -> Vec<String> {
        self.choices.clone()
    }

    fn get_ballot_form(&self) -> BallotForm {
        BallotForm::Uninominal
    }
}
