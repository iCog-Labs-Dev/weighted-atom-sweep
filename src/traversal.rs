use crate::sweep::{AtomHeader, AtomPosition};
use pathmap::zipper::ReadZipperTracked;
use std::error::Error;

pub trait TransversalEngine<H: AtomHeader> {
    fn next_atom(&self, zipper: ReadZipperTracked<H>) -> Result<AtomPosition, impl Error>;
}
