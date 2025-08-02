use crate::voting_system::definition::*;

pub(crate) const NAME: &str = "plurality";

/// # First-past-the-post voting
///
/// Here an exemple :
/// ```rust
/// use libvotal::voting_system::plurality;
///
/// let mut p =
///     plurality(vec![String::from("A"), String::from("B"), String::from("C")].into_iter());
///
/// p.vote("A");
/// p.vote("B");
/// p.vote("C");
/// p.vote("A");
///
/// assert_eq!("A", p.result());
/// ```
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plurality_voting() {
        let mut p =
            plurality(vec![String::from("A"), String::from("B"), String::from("C")].into_iter());

        for v in vec!["A", "B", "A", "C", "B", "A"] {
            p.vote(v).unwrap();
        }

        assert_eq!("A", p.result());
    }
}
