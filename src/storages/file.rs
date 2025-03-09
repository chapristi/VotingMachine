use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::{
	fs::File,
	io::{AsyncReadExt, AsyncWriteExt},

};
use std::collections::BTreeMap as Map;
use std::collections::BTreeSet as Set;
use crate::domain::Candidate;
use crate::domain::Score;
use crate::domain::Scoreboard;
use crate::domain::Voter;
use crate::{domain::VotingMachine, storage::Storage};
use crate::domain::AttendenceSheet;
pub struct FileStore{
    filepath: String
}
const FILEPATH : &str = "machine.json";

impl FileStore{
    
    pub async fn create(machine: VotingMachine, filepath: &str) -> anyhow::Result<Self> {
        let mut file = match File::open(filepath).await {
            Ok(file) => file,
            Err(_) => File::create(filepath).await?
        };

		let voting_machine_json = serde_json::to_string(&VotingMachineDao::from(machine))?;
		file.write_all(voting_machine_json.as_bytes()).await?;

        Ok(Self {
            filepath: filepath.to_string(),
        })
    }
    
}
#[derive(Serialize, Deserialize)]
struct ScoreboardDao{
    scores : Map<String, usize>,
    blank_score : usize,
    invalid_score: usize,
}
#[derive(Serialize, Deserialize)]
pub struct VotingMachineDao{
   voters: Set<String>,
   scoreboard: ScoreboardDao,
}
impl From<Scoreboard> for ScoreboardDao {
    fn from(scoreboard :  Scoreboard) -> Self
    {
        let mut scores = Map::new();

        for (candidate, score) in scoreboard.scores{
            scores.insert(candidate.0, score.0);
        }
        Self{
            blank_score : scoreboard.blank_score.0,
            invalid_score: scoreboard.invalid_score.0,
            scores : scores,

        }
    }
}

impl From<ScoreboardDao> for Scoreboard {
    fn from(scoreboard :  ScoreboardDao) -> Self
    {
        let mut scores = Map::new();

        for (candidate, score) in scoreboard.scores{
            scores.insert(Candidate(candidate), Score(score));
        }
        Self{
            blank_score : Score(scoreboard.blank_score),
            invalid_score: Score(scoreboard.invalid_score),
            scores : scores,

        }
    }
}

impl From<VotingMachine> for VotingMachineDao {
    fn from(voting_machine :  VotingMachine) -> Self
    {
        let mut  voters = Set::new();

        for voter in voting_machine.get_voters().clone().0 {
            voters.insert(voter.0);
        }

        Self{
            voters : voters,
            scoreboard: ScoreboardDao::from(voting_machine.get_scoreboard().clone())
        }
    }
}

impl From<VotingMachineDao> for VotingMachine{
    fn from(voting_machine :  VotingMachineDao) -> Self
    {
        let mut voters = Set::new();

        for voter in voting_machine.voters {
            voters.insert(Voter(voter));
        }

        VotingMachine::recover_from(
     AttendenceSheet(voters),
            Scoreboard::from(voting_machine.scoreboard)
        )
    }
}


#[async_trait]
impl Storage for FileStore {
    async fn new(machine: VotingMachine) -> anyhow::Result<Self>
    {
        Self::create(machine, FILEPATH).await
    }

    async fn get_voting_machine(&self) -> anyhow::Result<VotingMachine> {
        let mut my_file = File::open(self.filepath.as_str()).await?;

        let mut my_slice = vec![];
        my_file.read_to_end(&mut my_slice).await?;

        let my_object: VotingMachineDao = serde_json::from_slice(&my_slice)?;

        Ok(VotingMachine::from(my_object))
    }
    
    async fn put_voting_machine(&mut self, machine: VotingMachine) -> anyhow::Result<()> {
		let mut file = File::create(self.filepath.clone()).await?;
		let voting_machine_json = serde_json::to_string(&VotingMachineDao::from(machine))?;
		file.write_all(voting_machine_json.as_bytes()).await?;
		Ok(())
	}
}


#[cfg(test)]
mod tests {
    use crate::domain::Candidate;

    use super::*;

    #[tokio::test]
    async fn test_get_return_what_we_inserted() {

        let voting_machine :  VotingMachine = VotingMachine::new(vec![Candidate(String::from("Louis"))]);

        let mut store: FileStore  = FileStore::new(voting_machine.clone()).await.expect("Erreur lors de la creation de la memoire");

        store.put_voting_machine(voting_machine.clone()).await.expect("Erreur lors de l'insertion de la machine");

        let expected_machine = store.get_voting_machine().await.expect("err lors de la recuperation de la machine");

        assert_eq!(expected_machine, voting_machine);
    }



}