use std::string::String;

#[derive(Debug)]
pub enum Error {
    Shallow(String),
    UnShallow(String)
}


impl ToString for Error {
    fn to_string(&self) -> String{
        match self {
            Error::Shallow(s) => format!("Shallow {}",s),
            Error::UnShallow(s) => format!("UnShallow {}",s)
        }
    }
}