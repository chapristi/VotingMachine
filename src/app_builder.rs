use crate::configuration::Configuration;
use crate::configuration::Language;
use crate::configuration::ServiceType;
use crate::configuration::StorageType;
use crate::domain::Candidate;
use crate::domain::VotingMachine;
use crate::interfaces::lexicon::Lexicon;
use crate::interfaces::lexicons::english::ENGLISH;
use crate::interfaces::lexicons::french::FRENCH;
use crate::services::service::Service;
use crate::services::stdio::StdioService;
use crate::services::tcp::TcpService;
use crate::services::udp::UdpService;
use crate::storage::Storage;
use crate::storages::file::FileStore;
use crate::storages::memory::MemoryStore;
use crate::use_cases::VotingController;


fn create_voting_machine(configuration: &Configuration) -> VotingMachine {
    let mut candidates: Vec<Candidate> = vec![];

    for candidate in &configuration.candidates {
        candidates.push(Candidate(candidate.clone()));
    }

    VotingMachine::new(candidates)
}

pub async fn handle_lines<Store: Storage+Sync+Send, Serv: Service<Store>>(config: Configuration) -> anyhow::Result<()> {

    let voting_machine: VotingMachine = create_voting_machine(&config);
    let lexicon: Lexicon = match config.language {
        Language::FR => {
           FRENCH
        },
        Language::EN => {
            ENGLISH
        }
    };
    let store = Store::new(voting_machine).await?;
    let controller  = VotingController::new(store);

    let port = config.port.unwrap_or(9999);
    Serv::new(port, lexicon, controller)
		.serve()
		.await?;
    
    Ok(())
}

pub async fn run_app(config: Configuration) -> anyhow::Result<()> 
{
    match config.storage_type {
        StorageType::File => {
            dispatch_service::<FileStore>(config).await
        },
        StorageType::Memory => {
            dispatch_service::<MemoryStore>(config).await
        }
    }
}

async fn dispatch_service<Store: Storage + Send+ Sync+ Clone+ 'static>(config: Configuration)->Result<(), anyhow::Error>
{

    match config.service {
        ServiceType::STDIO =>{
            handle_lines::<Store, StdioService<Store>>(config).await
        }
        ServiceType::UDP => {
            handle_lines::<Store, UdpService<Store>>(config).await
        }
        ServiceType::TCP => {
            handle_lines::<Store, TcpService<Store>>(config).await

        }
    }
}