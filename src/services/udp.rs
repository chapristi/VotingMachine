use std::str;

use async_trait::async_trait;

use crate::{interfaces::{cli_interface::handle_line, lexicon::Lexicon}, storage::Storage, use_cases::VotingController};

use super::service::Service;
use tokio::net::UdpSocket;

pub struct UdpService<Store>
{
    port : u16,
    lexicon: Lexicon,
    controller : VotingController<Store>
}

#[async_trait]
impl <Store : Storage + Send + Sync> Service<Store> for UdpService<Store>{

    fn new(port:u16,lexicon:Lexicon,controller:VotingController<Store>) -> Self {
        Self { port: port, lexicon: lexicon, controller: controller }
    }


    async fn serve(&self) -> Result<(), anyhow::Error>
    {
        let url = format!("127.0.0.1:{}",self.port);
        let socket = UdpSocket::bind(url).await?;
    
        let mut buffer = vec![0u8; 1024];
    
        loop {
            let (size, sender) = socket.recv_from(&mut buffer).await?;
            let received_message = str::from_utf8(&buffer[0..size])?;
            println!("Received '{}' from {}", received_message.trim(), sender);
    
            let result = handle_line(received_message, &self.controller, &self.lexicon).await?;
            socket
                .send_to(result.as_bytes(), &sender)
                .await?;
        } 
       
    }
}