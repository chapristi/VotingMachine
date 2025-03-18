
use std::sync::Arc;

use serde::Deserialize;
use tokio::sync::RwLock;

use crate::{domain::{BallotPaper, Candidate, VoteOutcome, Voter, VotingMachine}, storage::Storage};

#[derive(Deserialize, Clone)]
pub struct VoteForm {
    pub voter : String,
    pub candidate: String,
}

impl From<VoteForm> for BallotPaper{
    fn from(vote_form: VoteForm) -> Self {
        let candidate = match vote_form.candidate.is_empty() {
            true => None,
            false => Some(Candidate(vote_form.candidate)),
        };
        
        Self{
            voter: Voter(vote_form.voter),
            candidate : candidate
        }
    }

}

#[derive(Clone)]
pub struct VotingController<Store>{
    store: Arc<RwLock<Store>>,
}
impl<Store: Storage> VotingController<Store> {
    pub fn new(store: Store) -> Self {
        Self { store: Arc::new(RwLock::new(store)) }
    }

    pub async fn vote(&self, vote_form: VoteForm) -> anyhow::Result<VoteOutcome> {
        let mut store = self.store.write().await;
        
        let mut voting_machine = store.get_voting_machine().await?;

        let outcome = voting_machine.vote(BallotPaper::from(vote_form));

        store.put_voting_machine(voting_machine).await?;

        Ok(outcome)
    }

    pub async fn get_voting_machine(&self) -> anyhow::Result<VotingMachine> {
        let store = self.store.read().await;
        store.get_voting_machine().await
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::{domain::{Score, Scoreboard}, storages::memory::MemoryStore};

    use super::*;

    #[tokio::test]
    async fn test_accepted_vote() -> anyhow::Result<()> {
        let candidates = vec![Candidate(String::from("Louis"))];
        let voting_machine: VotingMachine = VotingMachine::new(candidates);
        let store = MemoryStore::new(voting_machine).await.expect("probleme lors de l'instanciation de la memoire");

        
        let voting_controller  = VotingController::new(store);


        let vote_form =  VoteForm{
            voter: String::from("Louis"),
            candidate: String::from("Louis")
        };
        let mut correct_scores = BTreeMap::new();
		correct_scores.insert(Candidate(String::from("Louis")), Score(1));

        let correct_scoreboard = Scoreboard{
            
                scores: correct_scores,
                blank_score: Score(0),
                invalid_score: Score(0),
            
        };
        let result = voting_controller.vote(vote_form).await.expect("err lors du vote");
        let voting_machine = voting_controller.get_voting_machine().await.expect("erreur lors de la recuperation");
        assert_eq!(result,VoteOutcome::AcceptedVote(Voter( String::from("Louis")), Candidate(String::from("Louis"))));
        assert_eq!(correct_scoreboard,voting_machine.get_scoreboard().clone());
        Ok(())


    }
    #[tokio::test]
    async fn test_blank_vote() -> anyhow::Result<()> {
        let candidates = vec![Candidate(String::from("Louis"))];
        let voting_machine: VotingMachine = VotingMachine::new(candidates);
        let store = MemoryStore::new(voting_machine).await.expect("probleme lors de l'instanciation de la memoire");
        let voting_controller  = VotingController::new(store);


        let vote_form =  VoteForm{
            voter: String::from("Louis"),
            candidate: String::from("")
        };
		
        let mut correct_scores = BTreeMap::new();
		correct_scores.insert(Candidate(String::from("Louis")), Score(0));

        let correct_scoreboard = Scoreboard{
            
                scores: correct_scores,
                blank_score: Score(1),
                invalid_score: Score(0),
            
        };
        let result = voting_controller.vote(vote_form).await.expect("err lors du vote");
        let voting_machine = voting_controller.get_voting_machine().await.expect("erreur lors de la recuperation");
        assert_eq!(result,VoteOutcome::BlankVote(Voter( String::from("Louis"))));
        assert_eq!(correct_scoreboard,voting_machine.get_scoreboard().clone());
        Ok(())


    }
    #[tokio::test]
    async fn test_null_vote() -> anyhow::Result<()> {
        let candidates = vec![Candidate(String::from("Louis"))];
        let voting_machine: VotingMachine = VotingMachine::new(candidates);
        let store = MemoryStore::new(voting_machine).await.expect("probleme lors de l'instanciation de la memoire");
        let voting_controller: VotingController<_>  = VotingController::new(store);


        let vote_form =  VoteForm{
            voter: String::from("Louis"),
            candidate: String::from("Jeane oscour")
        };
        let mut correct_scores = BTreeMap::new();
		correct_scores.insert(Candidate(String::from("Louis")), Score(0));

        let correct_scoreboard = Scoreboard{
            
                scores: correct_scores,
                blank_score: Score(0),
                invalid_score: Score(1),
            
        };
        let result = voting_controller.vote(vote_form).await.expect("err lors du vote");
        let voting_machine = voting_controller.get_voting_machine().await.expect("erreur lors de la recuperation");
        assert_eq!(result,VoteOutcome::InvalidVote(Voter( String::from("Louis"))));
        assert_eq!(correct_scoreboard,voting_machine.get_scoreboard().clone());
        Ok(())
    }


    #[tokio::test]
    async fn test_has_already_voted() -> anyhow::Result<()> {
        let candidates = vec![Candidate(String::from("Louis"))];
        let voting_machine: VotingMachine = VotingMachine::new(candidates.clone());
        let store = MemoryStore::new(voting_machine).await.expect("probleme lors de l'instanciation de la memoire");
        let voting_controller  = VotingController::new(store);


        let vote_form =  VoteForm{
            voter: String::from("Louis"),
            candidate: String::from("Jeane oscour")
        };
        let mut correct_scores = BTreeMap::new();
		correct_scores.insert(Candidate(String::from("Louis")), Score(0));

        let correct_scoreboard = Scoreboard{
            
                scores: correct_scores,
                blank_score: Score(0),
                invalid_score: Score(1),
            
        };
        voting_controller.vote(vote_form.clone()).await.expect("err lors du vote");
        let result = voting_controller.vote(vote_form.clone()).await.expect("err lors du vote");

        let voting_machine = voting_controller.get_voting_machine().await.expect("erreur lors de la recuperation");
        assert_eq!(result,VoteOutcome::HasAlreadyVoted(Voter(String::from("Louis"))));
        assert_eq!(correct_scoreboard,voting_machine.get_scoreboard().clone());
        Ok(())
    }
}