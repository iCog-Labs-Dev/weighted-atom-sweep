use crate::sweep::{ AtomHeader, WeightedValue };
use pathmap::zipper::{ ZipperCreation, ZipperHeadOwned, ZipperMoving, ZipperValues, ZipperWriting };
use std::{ops::Deref, sync::Arc};

pub struct WeightedMap<H: AtomHeader + Default> {
    pub inner: Arc<ZipperHeadOwned<WeightedValue<H>>>,
}

impl<H: AtomHeader + Default> WeightedMap<H> {
     
    pub fn new(map: ZipperHeadOwned<WeightedValue<H>>) -> Self {
        
        Self {
            inner: Arc::new(map),
        }
    }

    pub fn get_val(&self, path: &[u8]) -> Option<WeightedValue<H>> {

        match self.inner.read_zipper_at_path(path) {
            Ok(reader) => reader.val().cloned(),
            Err(_) => None 
        }
    }

    pub fn set_val(&self, path: &[u8], val: WeightedValue<H>) -> () {

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

    pub fn set_weighted_val(&self, path: &[u8], val: H) -> Result<(), &'static str> {
        let current_weighted = self.get_val(path).unwrap_or(WeightedValue::default());

        // Update the leaf value
        self.set_val(path, WeightedValue { 
            val: val.clone(),
            child_agg_w: current_weighted.child_agg_w.clone()
        });

        // Propagate weight changes up to root
        self.propagate(path, val)
        
    }

    fn propagate(&self, path: &[u8], new_val:H) -> Result<(), &'static str>
    {
        if path.is_empty() {
            return Ok(()); // At root, noting to propagate
        }

        // create a read zipper to traverse up from the changed node
        let mut read_zipper = self.inner.write_zipper_at_exclusive_path(&[])
            .map_err(|_| "Failed to get read zipper")?;

        read_zipper.descend_to(path);

        while read_zipper.ascend_until_branch() {

            if let Some(parent_weighted) = read_zipper.val() {
                read_zipper.set_val( WeightedValue {
                    val: parent_weighted.val.clone(),
                    child_agg_w: parent_weighted.child_agg_w.add(&new_val),
                });
            } else {
                read_zipper.set_val( WeightedValue {
                    val: H::default(),
                    child_agg_w: new_val.clone(),
                });
            }
        }
        Ok(())

    }

}

impl<H> Deref for WeightedMap<H>
where
    H: AtomHeader + Default,
{
    type Target = ZipperHeadOwned<WeightedValue<H>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
