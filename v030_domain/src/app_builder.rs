use crate::{
    configuration::Configuration,
    domain::{BallotPaper, Candidate, VoteOutcome, Voter, VotingMachine},
};
use tokio::io::{self, AsyncBufReadExt, BufReader};

fn commands() {
    println!("Commands :");
    println!(" - vote : to vote");
    println!(" - voters : see all voters");
    println!(" - scores : see scores");
    println!(" - q : quit");
}

fn create_voting_machine(conf: &Configuration) -> VotingMachine {
    let mut candidates: Vec<Candidate> = Vec::new();

    for candidate in conf.candidates.clone() {
        candidates.push(Candidate(candidate));
    }

    VotingMachine::new(candidates)
}

pub async fn run_app(conf: Configuration) -> anyhow::Result<()> {
    let mut voting_machine: VotingMachine = create_voting_machine(&conf);

    println!(" ~ Welcome ~");
    commands();
    println!("Please type a command :");

    let mut lines = BufReader::new(io::stdin()).lines();

    while let Some(input) = lines.next_line().await? {
        match input.split_whitespace().next().unwrap_or("") {
            "vote" => {
                let mut ite = input.split_whitespace();
                ite.next();
                match ite.next() {
                    Some(voter_str) => {
                        let voter = Voter(voter_str.to_string());
                        let candidate = match ite.next() {
                            Some(candidate_str) => Some(Candidate(candidate_str.to_string())),
                            None => None,
                        };
                        let ballot_paper = BallotPaper {
                            voter: voter,
                            candidate: candidate,
                        };
                        match voting_machine.vote(ballot_paper) {
                            VoteOutcome::AcceptedVote(voter, candidate) => {
                                println!("{} voted for {} !", voter.0, candidate.0)
                            }
                            VoteOutcome::BlankVote(voter) => println!("{} voted blank !", voter.0),
                            VoteOutcome::InvalidVote(voter) => println!("{} voted null !", voter.0),
                            VoteOutcome::HasAlreadyVoted(voter) => {
                                println!("{} already voted !", voter.0)
                            }
                        }
                    }
                    None => println!("Command \"vote\" gets 1 to 2 args <voter> [<candidate>]"),
                }
            }
            "voters" => println!("{}", voting_machine.get_voters().print()),
            "scores" => println!("{}", voting_machine.get_scoreboard().print()),
            "q" => {
                println!("Goodbye !");
                break;
            }
            "" => commands(),
            _ => println!("Command \"{}\" not known...", input),
        }
        println!("\nPlease type a command :");
    }

    Ok(())
}
