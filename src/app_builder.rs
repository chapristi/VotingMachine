use crate::configuration::Configuration;
use crate::configuration::Language;
use crate::configuration::StorageType;
use crate::domain::Candidate;
use crate::domain::VotingMachine;
use crate::interfaces::cli_interface::handle_line;
use crate::interfaces::lexicon::Lexicon;
use crate::storage::Storage;
use crate::storages::file::FileStore;
use crate::storages::memory::MemoryStore;
use crate::use_cases::VotingController;

use tokio::io::{self, AsyncBufReadExt, BufReader};


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
    let lexicon: Lexicon = match config.language {
        Language::FR => {
            Lexicon::french()
        },
        Language::EN => {
            Lexicon::english()
        }
    };
    
    let mut controller  = VotingController::new(store);

    let mut lines = BufReader::new(io::stdin()).lines();

    while let Some(line) = lines.next_line().await? {
      println!("{}",handle_line(line.as_str(), &mut controller, &lexicon).await?);
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
