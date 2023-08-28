use std::io::Error;

pub trait PackMod {
    fn remove_package(name: String) -> Result<(), Error>;
}
