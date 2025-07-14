pub enum BallotForm {
    Uninominal,
}

#[allow(dead_code)]
pub trait Voting {
    fn get_name(&self) -> String;
    fn get_choices(&self) -> Vec<String>;
    fn get_ballot_form(&self) -> BallotForm;
}
