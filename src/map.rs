use crate::sweep::AtomHeader;
use pathmap::zipper::ZipperHeadOwned;
use std::{ops::Deref, sync::Arc};

pub struct WeightedMap<H: AtomHeader> {
    pub inner: Arc<ZipperHeadOwned<H>>,
}

impl<H> Deref for WeightedMap<H>
where
    H: AtomHeader,
{
    type Target = ZipperHeadOwned<H>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
