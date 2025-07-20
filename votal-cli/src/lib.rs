use libvotal::voting_system::*;

pub struct ConfigServer {
    pub name: String,
}

impl ConfigServer {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<(ConfigServer, VotingSystem), &'static str> {
        // program's name
        args.next();

        let server_name = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a server name"),
        };

        let vote = match args.next() {
            Some(arg) => find_voting_system(&arg),
            None => return Err("Didn't get a voting system name"),
        };

        let mut choices: Vec<String> = vec![];

        for c in args {
            choices.push(c);
        }

        Ok((ConfigServer{name: server_name}, vote(choices)))
    }
}
