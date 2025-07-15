/// Describe the ballot form
pub enum BallotForm {
    Uninominal,
}

/// Used to describe election
#[allow(dead_code)]
pub trait Voting {
    /// Give the name of the voting system
    fn get_name(&self) -> String;
    /// Give all choices
    fn get_choices(&self) -> impl Iterator<Item = &String>;
    /// Give the ballot form
    fn get_ballot_form(&self) -> BallotForm;
    /// Just vote
    fn vote(&mut self, ballot: String) -> Result<(), &'static str>;
    /// Calculate the election result
    fn result<'a>(&'a self) -> &'a str;
}
