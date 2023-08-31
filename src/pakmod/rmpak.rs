use std::io::Error;

pub trait PackMod {
    fn remove_package(&mut self, name: String) -> Result<(), Error>;
    fn save(&self) -> Result<(), Error>;
}
