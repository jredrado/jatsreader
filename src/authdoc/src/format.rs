use std::fmt;

pub trait PartialDisplay {
    fn start_fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error>;
    fn end_fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error>;
}