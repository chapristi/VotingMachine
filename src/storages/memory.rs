use anyhow::Ok;
use async_trait::async_trait;
use crate::{domain::VotingMachine, storage::Storage};

#[derive(Clone)]
pub struct MemoryStore{
   machine: VotingMachine,
}

#[async_trait]
impl Storage for MemoryStore {
    async fn new(machine: VotingMachine) -> anyhow::Result<Self>
    {
        Ok(Self { machine })
    }

    async fn get_voting_machine(&self) -> anyhow::Result<VotingMachine>
    {
        Ok(self.machine.clone())
    }
    async fn put_voting_machine(&mut self, machine: VotingMachine) -> anyhow::Result<()>
    {
        self.machine = machine;
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

        let mut store : MemoryStore = MemoryStore::new(voting_machine.clone()).await.expect("Erreur lors de la creation de la memoire");

        store.put_voting_machine(voting_machine.clone()).await.expect("Erreur lors de l'insertion de la machine");

        let expected_machine = store.get_voting_machine().await.expect("err lors de la recuperation de la machine");

        assert_eq!(expected_machine, voting_machine);
    }

    #[tokio::test]
    async fn test_keep_file_informations_between_many_instance() {

        let voting_machine :  VotingMachine = VotingMachine::new(vec![Candidate(String::from("Louis"))]);

        let mut store1 : MemoryStore = MemoryStore::new(voting_machine.clone()).await.expect("Erreur lors de la creation de la memoire");
        let store2 : MemoryStore = MemoryStore::new(voting_machine.clone()).await.expect("Erreur lors de la creation de la memoire");

        store1.put_voting_machine(voting_machine.clone()).await.expect("Erreur lors de l'insertion de la machine");

        let machine2 = store1.get_voting_machine().await.expect("err lors de la recuperation de la machine dans l'instance 1");
        let machine1 = store2.get_voting_machine().await.expect("err lors de la recuperation de la machine dans l'ibstance 2");

        assert_eq!(machine1, machine2);
    }
}