use anyhow::Error;

pub trait CommandsAction {
    fn run(&self) -> Result<(), Error>;
}