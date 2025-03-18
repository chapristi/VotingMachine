use async_trait::async_trait;
use tokio::{io::{AsyncBufReadExt, AsyncWriteExt, BufReader}, net::TcpListener};
use crate::{interfaces::{cli_interface::handle_line, lexicon::Lexicon}, storage::Storage, use_cases::VotingController};
use super::service::Service;

pub struct TcpService<Store> {
    port: u16,
    lexicon: Lexicon,
    controller: VotingController<Store>,
}

#[async_trait]
impl<Store: Storage + Send + Sync + Clone + 'static> Service<Store> for TcpService<Store> {
    fn new(port: u16, lexicon: Lexicon, controller: VotingController<Store>) -> Self {
        Self { port, lexicon, controller }
    }

    async fn serve(&self) -> Result<(), anyhow::Error> {
        let endpoint = format!("127.0.0.1:{}", self.port);
        let listener = TcpListener::bind(&endpoint).await?;
        
        println!("TCP server listening on {}", endpoint);

        loop {
            let (mut stream, _) = listener.accept().await?;
            
            let lexicon = self.lexicon.clone();
            let mut controller = self.controller.clone();

            tokio::spawn(async move {
                let (reader, mut writer) = stream.split();
                let mut lines = BufReader::new(reader).lines();

                loop {
                    match lines.next_line().await {
                        Ok(Some(line)) => {
                            match handle_line(line.as_str(), &mut controller, &lexicon).await {
                                Ok(response) => {
                                    if let Err(e) = writer.write_all(response.as_bytes()).await {
                                        eprintln!("Erreur d'écriture TCP : {}", e);
                                        break;
                                    }
                                    if let Err(e) = writer.flush().await {
                                        eprintln!("Erreur de flush TCP : {}", e);
                                        break;
                                    }
                                }
                                Err(e) => {
                                    eprintln!("Erreur de traitement : {}", e);
                                    break;
                                }
                            }
                        }
                        Ok(None) => break,
                        Err(e) => {
                            eprintln!("Erreur de lecture TCP : {}", e);
                            break;
                        }
                    }
                }
            });
        }
    }
}
