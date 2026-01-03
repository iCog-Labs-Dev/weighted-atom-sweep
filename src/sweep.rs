use crate::map::WeightedMap;
use crate::operation::{Operation, OperationObserver};
use crate::traversal::TransversalEngine;
// use pathmap::PathMap;
use pathmap::zipper::{ZipperCreation, ZipperHeadOwned};
use std::sync::{Arc, mpsc};

pub type AtomPosition = Vec<u8>;

pub trait AtomHeader: std::fmt::Debug + Clone + Send + Sync + Unpin + 'static + Default {
    fn add(&self, other: &Self) -> Self; 
}
pub trait KernelOperation<H: AtomHeader>:
    Operation<H> + Send + Sync + Clone + std::fmt::Debug + PartialEq + 'static
{
}

pub trait SweepTransversalEngine<H: AtomHeader>:
    for<'a> TransversalEngine<H> + Send + Sync + Clone + std::fmt::Debug + 'static
{
}

pub struct WeightedAtomSweepSettings {}

#[derive(Clone, Debug, Default)]
pub struct WeightedValue<H: AtomHeader> {
    pub val: H,
    pub child_agg_w: H
}

impl <H: AtomHeader> AtomHeader for WeightedValue<H> {

    fn add(&self, other: &Self) -> Self {
        WeightedValue {
            val: self.val.clone(),
            child_agg_w: self.child_agg_w.add(&other.val),
        }
    }
}

pub struct WeightedAtomSweep<T, O, H>
where
    H: AtomHeader,
    T: SweepTransversalEngine<WeightedValue<H>>,
    O: KernelOperation<WeightedValue<H>>,
{
    // pub reciever: mpsc::Receiver<T::Atom>,
    pub traversal: Arc<T>,
    pub operations: Vec<O>,
    pub settings: WeightedAtomSweepSettings,
    pub map: WeightedMap<H>,
}

impl<T, O, H> WeightedAtomSweep<T, O, H>
where
    H: AtomHeader,
    T: SweepTransversalEngine<WeightedValue<H>>,
    O: KernelOperation<WeightedValue<H>>,
{
    pub fn new(traversal: T, operations: Vec<O>, settings: WeightedAtomSweepSettings, map: WeightedMap<H>) -> Self {
        Self {
            traversal: Arc::new(traversal),
            operations: operations,
            settings,
            map
        }
    }

    // TODO: map can be limited to a subset of the map
    pub fn spawn(self) -> Arc<ZipperHeadOwned<WeightedValue<H>>> {
        let (atom_sender, atom_reciever) = mpsc::channel::<AtomPosition>();
        let engine = self.traversal.clone();
        let sender = atom_sender.clone();
        let map = self.map.inner.clone();

        // spawn traversal thread
        std::thread::spawn(move || {
            // get access to a read zipper
            let traverse_zp = match self.map.read_zipper_at_borrowed_path(&[]) {
                Ok(zipper) => zipper,
                Err(_) => return,
            };

            let atom_path = engine.next_atom(traverse_zp).unwrap();
            sender.send(atom_path).unwrap();
        });

        // handle traversed atom
        std::thread::spawn(move || {
            let operations = self.operations.clone();
            // get access to a read zipper
            let atom = Arc::new(atom_reciever.recv().unwrap());

            for op in operations {
                op.transform(atom.clone().into());
            }
        });

        map
    }
}

impl<T, O, V> OperationObserver<WeightedValue<V>, O> for WeightedAtomSweep<T, O, V>
where
    V: AtomHeader,
    T: SweepTransversalEngine<WeightedValue<V>>,
    O: KernelOperation<WeightedValue<V>>,
{
    fn subscribe(&mut self, operation: O) {
        self.operations.push(operation);
    }

    fn unsubscribe(&mut self, operation: O) {
        self.operations.retain(|op| op != &operation);
    }
}
