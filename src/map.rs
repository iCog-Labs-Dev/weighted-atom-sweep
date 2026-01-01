use crate::sweep::AtomHeader;
use pathmap::zipper::{ ZipperCreation, ZipperHeadOwned, ZipperValues, ZipperWriting };
use std::{ops::Deref, sync::Arc};

pub struct WeightedMap<H: AtomHeader> {
    pub inner: Arc<ZipperHeadOwned<H>>,
}

impl<H:AtomHeader> WeightedMap<H> {
    
    pub fn new(map: ZipperHeadOwned<H>) -> Self {
        
        Self {
            inner: Arc::new(map),
        }
    }

    pub fn get_val(&self, path: &[u8]) -> Option<H> {

        match self.inner.read_zipper_at_path(path) {
            Ok(reader) => reader.val().cloned(),
            Err(_) => None 
        }
    }

    pub fn set_val(&self, path: &[u8], val: H) -> () {

        match self.inner.write_zipper_at_exclusive_path(path) {
            Ok(mut write_zipper) => {
                write_zipper.set_val(val);
                self.inner.cleanup_write_zipper(write_zipper);
            }
            Err(_) => {
                return;
            }
        };
    }
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
