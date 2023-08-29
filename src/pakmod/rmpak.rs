use std::io::Error;

pub trait PackMod {
    fn remove_package(&self, name: String) -> Result<(), Error>;
}
