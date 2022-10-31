use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Action {

    AddBlock,
    ValidateBlock{
        #[structopt()]
        index: usize,

    },
    ValidateBlockchain,
    CreateWallet,
    CreateTransaction,
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "The New Blockchain",
    about = "Command Line Application"
)]
pub struct CommandLineArgs{
    #[structopt(subcommand)]
    pub action: Action,

}