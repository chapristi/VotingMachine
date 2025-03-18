use async_trait::async_trait;
use tokio::io::{self, AsyncBufReadExt, BufReader};

use crate::{interfaces::{cli_interface::handle_line, lexicon::Lexicon}, storage::Storage, use_cases::VotingController};

use super::service::Service;

pub struct StdioService<Store>
{
    lexicon: Lexicon,
    controller : VotingController<Store>
}

#[async_trait]
impl <Store : Storage + Send + Sync> Service<Store> for StdioService<Store>{

    fn new(port:u16,lexicon:Lexicon,controller:VotingController<Store>) -> Self {
        Self { lexicon: lexicon, controller: controller }
    }

    async fn serve(&self) -> Result<(), anyhow::Error>
    {
        let mut lines = BufReader::new(io::stdin()).lines();

        while let Some(line) = lines.next_line().await? {
          println!("{}",handle_line(line.as_str(), &self.controller, &self.lexicon).await?);
        }
        Ok(())
    }
}