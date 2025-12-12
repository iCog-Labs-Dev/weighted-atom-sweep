use crate::map::WeightedMap;
use crate::operation::{Operation, OperationObserver};
use crate::traversal::TransversalEngine;
use pathmap::PathMap;
use pathmap::zipper::{ZipperCreation, ZipperHeadOwned};
use std::sync::{Arc, mpsc};

pub type AtomPosition = Vec<u8>;

pub trait AtomHeader: std::fmt::Debug + Clone + Send + Sync + Unpin + 'static {}
pub trait KernelOperation<H: AtomHeader>:
    Operation<H> + Send + Sync + Clone + std::fmt::Debug + PartialEq + 'static
{
}

pub trait SweepTransversalEngine<H: AtomHeader>:
    for<'a> TransversalEngine<H> + Send + Sync + Clone + std::fmt::Debug + 'static
{
}

pub struct WeightedAtomSweepSettings {}

pub struct WeightedAtomSweep<T, O, H>
where
    T: SweepTransversalEngine<H>,
    O: KernelOperation<H>,
    H: AtomHeader,
{
    // pub reciever: mpsc::Receiver<T::Atom>,
    pub traversal: Arc<T>,
    pub operations: Vec<O>,
    pub settings: WeightedAtomSweepSettings,
    pub map: WeightedMap<H>,
}

impl<T, O, H> WeightedAtomSweep<T, O, H>
where
    T: SweepTransversalEngine<H>,
    O: KernelOperation<H>,
    H: AtomHeader,
{
    pub fn new(traversal: T, operations: Vec<O>, settings: WeightedAtomSweepSettings) -> Self {
        Self {
            traversal: Arc::new(traversal),
            operations: operations,
            settings,
            map: WeightedMap {
                inner: Arc::new(PathMap::<H>::new().into_zipper_head([])),
            },
        }
    }

    // TODO: map can be limited to a subset of the map
    pub fn spawn(self) -> Arc<ZipperHeadOwned<H>> {
        let (atom_sender, atom_reciever) = mpsc::channel::<AtomPosition>();
        let engine = self.traversal.clone();
        let sender = atom_sender.clone();
        let map = self.map.inner.clone();

        // spawn traversal thread
        std::thread::spawn(move || {
            // get access to a read zipper
            let traverse_zp = self.map.read_zipper_at_borrowed_path(&[]).unwrap();

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

impl<T, O, H> OperationObserver<H, O> for WeightedAtomSweep<T, O, H>
where
    T: SweepTransversalEngine<H>,
    O: KernelOperation<H>,
    H: AtomHeader,
{
    fn subscribe(&mut self, operation: O) {
        self.operations.push(operation);
    }

    fn unsubscribe(&mut self, operation: O) {
        self.operations.retain(|op| op != &operation);
    }
}
