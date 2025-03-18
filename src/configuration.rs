use clap::Parser;
use clap::ValueEnum;

#[derive(Clone,Copy, ValueEnum, Debug)]
pub enum StorageType {
    File,
    Memory,
}
#[derive(Clone,Copy, ValueEnum, Debug)]
pub enum Language {
    FR,
    EN,
}

#[derive(Clone,Copy, ValueEnum, Debug)]
pub enum ServiceType {
    STDIO,
    UDP,
    TCP
}


#[derive(Debug, Parser)]
pub struct Configuration {
    #[arg(short = 'c', long, required = true, num_args = 1..)]
    pub candidates: Vec<String>,

    #[arg(short = 's', long, required = true, num_args = 1)]
    pub storage_type: StorageType,

    #[arg(short = 'l', long, required = true, num_args = 1)]
    pub language: Language,

    #[arg(short = 'e', long, required = true, num_args = 1)]
    pub service: ServiceType,

    #[arg(short = 'p', long, required = false, num_args = 1)]
    pub port: Option<u16>,
}
