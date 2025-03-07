use std::collections::BTreeMap as Map;
use std::collections::BTreeSet as Set;

#[derive(Ord, PartialEq, Eq, PartialOrd, Clone, Debug)]
pub struct Voter(pub String);

#[derive(Ord, PartialEq, Eq, PartialOrd, Clone, Debug)]
pub struct Candidate(pub String);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Score(pub usize);

#[derive(Debug, Clone, Eq, PartialEq,)]
pub struct AttendenceSheet(pub Set<Voter>);

#[derive(Debug, Clone,Eq, PartialEq)]
pub struct Scoreboard {
    pub scores: Map<Candidate, Score>,
    pub blank_score: Score,
    pub invalid_score: Score,
}
#[derive(Clone)]
pub struct BallotPaper {
    pub voter: Voter,
    pub candidate: Option<Candidate>,
}
#[derive(Eq, PartialEq, Debug)]
pub enum VoteOutcome {
    AcceptedVote(Voter, Candidate),
    BlankVote(Voter),
    InvalidVote(Voter),
    HasAlreadyVoted(Voter),
}
#[derive( Clone, Debug, Eq, PartialEq, )]

pub struct VotingMachine {
    voters: AttendenceSheet,
    scoreboard: Scoreboard,
}

impl Scoreboard {
    pub fn new(candidates: Vec<Candidate>) -> Self {
        let mut scores: Map<Candidate, Score> = Map::new();

        for candidat in candidates {
            scores.insert(candidat, Score(0));
        }

        Self {
            scores: scores,
            blank_score: Score(0),
            invalid_score: Score(0),
        }
    }
}

impl VotingMachine {
    pub fn new(candidates: Vec<Candidate>) -> Self {
        Self {
            voters: AttendenceSheet(Set::new()),
            scoreboard: Scoreboard::new(candidates),
        }
    }

    pub fn recover_from(voters: AttendenceSheet, scoreboard :  Scoreboard)-> Self
    {
        Self{
            voters, scoreboard
        }
    }
    pub fn vote(&mut self, ballot_paper: BallotPaper) -> VoteOutcome {
        if self.voters.0.contains(&ballot_paper.voter) {
            return VoteOutcome::HasAlreadyVoted(ballot_paper.voter);
        }

        self.voters.0.insert(ballot_paper.voter.clone());

        match ballot_paper.candidate {
            Some(candidate) => match self.scoreboard.scores.get_mut(&candidate) {
                Some(score) => {
                    *score = Score(score.0 + 1);
                    VoteOutcome::AcceptedVote(ballot_paper.voter, candidate)
                }
                None => {
                    self.scoreboard.invalid_score = Score(self.scoreboard.invalid_score.0 + 1);
                    VoteOutcome::InvalidVote(ballot_paper.voter)
                }
            },
            None => {
                self.scoreboard.blank_score = Score(self.scoreboard.blank_score.0 + 1);
                VoteOutcome::BlankVote(ballot_paper.voter)
            }
        }
    }

    pub fn get_scoreboard(&self) -> &Scoreboard {
        &self.scoreboard
    }

    pub fn get_voters(&self) -> &AttendenceSheet {
        &self.voters
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> VotingMachine {
        let candidates = vec![
            Candidate(String::from("Biggard")),
            Candidate(String::from("Louis")),
        ];

        VotingMachine::new(candidates)
    }

    #[test]
    fn test_accepted_vote() {
        let mut voting_machine = setup();

        let ballot_paper = BallotPaper {
            voter: Voter(String::from("Louis")),
            candidate: Some(Candidate(String::from("Biggard"))),
        };

        let outcome = voting_machine.vote(ballot_paper);

        match outcome {
            VoteOutcome::AcceptedVote(_, candidate) => {
                assert_eq!(candidate.0, "Biggard");
            }
            _ => panic!("Expected AcceptedVote"),
        }
    }

    #[test]
    fn test_blank_vote() {
        let mut voting_machine = setup();

        let ballot_paper = BallotPaper {
            voter: Voter(String::from("John")),
            candidate: None,
        };

        let outcome = voting_machine.vote(ballot_paper);

        match outcome {
            VoteOutcome::BlankVote(voter) => {
                assert_eq!(voter.0, "John");
            }
            _ => panic!("Expected BlankVote"),
        }
    }

    #[test]
    fn test_invalid_vote() {
        let mut voting_machine = setup();

        let ballot_paper = BallotPaper {
            voter: Voter(String::from("John")),
            candidate: Some(Candidate(String::from("Invalid"))),
        };

        let outcome = voting_machine.vote(ballot_paper);

        match outcome {
            VoteOutcome::InvalidVote(voter) => {
                assert_eq!(voter.0, "John");
            }
            _ => panic!("Expected InvalidVote"),
        }
    }

    #[test]
    fn test_has_already_voted() {
        let mut voting_machine = setup();

        let ballot_paper_1 = BallotPaper {
            voter: Voter(String::from("Alice")),
            candidate: Some(Candidate(String::from("Biggard"))),
        };

        let ballot_paper_2 = BallotPaper {
            voter: Voter(String::from("Alice")),
            candidate: Some(Candidate(String::from("Louis"))),
        };

        voting_machine.vote(ballot_paper_1);

        let outcome = voting_machine.vote(ballot_paper_2);

        match outcome {
            VoteOutcome::HasAlreadyVoted(voter) => {
                assert_eq!(voter.0, "Alice");
            }
            _ => panic!("Expected HasAlreadyVoted"),
        }
    }
}
