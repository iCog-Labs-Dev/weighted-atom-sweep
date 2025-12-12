use std::sync::Arc;

use crate::sweep::{AtomHeader, AtomPosition};

pub trait Operation<H: AtomHeader> {
    fn name(&self) -> &str;
    fn transform(&self, zipper: Arc<AtomPosition>) -> ();
}

pub trait OperationObserver<H, O>
where
    H: AtomHeader,
    O: Operation<H>,
{
    fn subscribe(&mut self, observer: O);
    fn unsubscribe(&mut self, observer: O);
}
