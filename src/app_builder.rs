use crate::configuration::Configuration;
use crate::configuration::StorageType;
use crate::domain::Candidate;
use crate::domain::VoteOutcome;
use crate::domain::VotingMachine;
use crate::storage::Storage;
use crate::storages::file::FileStore;
use crate::storages::memory::MemoryStore;
use crate::use_cases::VoteForm;
use crate::use_cases::VotingController;

use tokio::io::{self, AsyncBufReadExt, BufReader};

fn display_menu() {
    println!("Il y a les 4 commandes suivantes :");
    println!("1) voter Tux Nixos pour voter Nixos en tant que Tux");
    println!("2) voter Tux sans mettre de vote pour voter blanc");
    println!("3) votants afficher la liste des votants");
    println!("4) scores fait afficher les scores pour les candidats");
}
fn create_voting_machine(configuration: &Configuration) -> VotingMachine {
    let mut candidates: Vec<Candidate> = vec![];

    for candidate in &configuration.candidates {
        candidates.push(Candidate(candidate.clone()));
    }

    VotingMachine::new(candidates)
}

pub async fn handle_lines<Store: Storage>(config: Configuration) -> anyhow::Result<()> {

    let voting_machine: VotingMachine = create_voting_machine(&config);
    let store = Store::new(voting_machine).await?;
    let mut voting_controller  = VotingController::new(store);

    let mut lines = BufReader::new(io::stdin()).lines();

    while let Some(line) = lines.next_line().await? {
        let mut words = line.trim().split_whitespace();
        let voting_machine: VotingMachine = voting_controller.get_voting_machine().await?;

        match words.next() {
            Some(command) => match command {
                "voter" => match words.next() {
                    Some(voter) => {
                        let candidate: String = match words.next() {
                            Some(word) => word.to_string(),
                            None => String::from(""),
                        };
                        let ballot_paper: VoteForm = VoteForm {
                            voter: voter.to_string().clone(),
                            candidate: candidate,
                        };

                        let vote: VoteOutcome = voting_controller.vote(ballot_paper).await?;

                        match vote {
                            VoteOutcome::InvalidVote(voter) => {
                                println!("le votant {:?} à voté null", voter)
                            }
                            VoteOutcome::BlankVote(voter) => {
                                println!("Le votant {:?} a voté blanc", voter)
                            }
                            VoteOutcome::HasAlreadyVoted(voter) => {
                                println!("le votant {:?} a deja voté", voter)
                            }
                            VoteOutcome::AcceptedVote(voter, candidat) => {
                                println!("le votant {:?} a voté pour {:?}", voter, candidat)
                            }
                        }
                    }
                    None => println!("Commande voter invalide"),
                },

                "scores" => println!("Scores actuels : {:?}", voting_machine.get_scoreboard()),
                "votants" => println!("Liste des votants : {:?}", voting_machine.get_voters()),
                _ => println!("Commande inconnue. Tapez une commande valide."),
            },
            None => display_menu(),
        }
    }
    Ok(())
}

pub async fn run_app(config: Configuration) -> anyhow::Result<()> 
{
    match config.storage_type {
        StorageType::File => {
            handle_lines::<FileStore>(config).await
        },
        StorageType::Memory => {
            handle_lines::<MemoryStore>(config).await
        }
    }
}
