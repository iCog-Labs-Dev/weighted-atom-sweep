use pathmap::zipper::{WriteZipperTracked, ZipperHeadOwned};
use std::sync::Arc;

type AtomHeader = f64;

pub enum AtomWeightTransformEvent {
    Transform(String), // Holds the transformed atom, if the current atom is not the transformed
    // atom then it means it is the ancestor of the transformed atom
    Explore(String),
    Export,
    // etc ...
}

pub struct TranversalEngine {
    pub traverse: Arc<dyn Fn(WriteZipperTracked<AtomHeader>) -> bool + Send + Sync>,
    pub visit: Arc<dyn Fn(WriteZipperTracked<AtomHeader>) -> () + Send + Sync>,
}

impl TranversalEngine {
    pub fn step(&self, zipper: WriteZipperTracked<AtomHeader>) -> () {
        let should_visit = (self.traverse)(zipper); // Execute the necessary logic

        if should_visit {
            // execute AtomWeightTransform.transform
            todo!()
        }
    }
}

pub struct AtomWeightTransform {
    pub event: Vec<AtomWeightTransformEvent>,
    // user defined logic to update the atom
    pub update_weight: Arc<
        dyn Fn(WriteZipperTracked<AtomHeader>, AtomWeightTransformEvent) -> Option<()>
            + Send
            + Sync,
    >,
    // settings for the atom weight changes
    pub interval: u64,
    pub timeout: u64,
    pub max_depth: u64,
    // etc ...
}

impl WeightedAtomSweep {
    pub fn with_process(
        map: ZipperHeadOwned<AtomHeader>,
        weight_transform: AtomWeightTransform,
    ) -> WeightedAtomSweep {
        // spawn a thread
        std::thread::spawn(move || {
            let mut engine = TranversalEngine {
                traverse: Arc::new(|zipper: WriteZipperTracked<AtomHeader>| {
                    // execute AtomWeightTransform.transform
                    todo!()
                }),
            };

            loop {
                engine.step(map);
            }
        });
    }
}

pub struct WeightedAtomSweep {
    pub map: Arc<ZipperHeadOwned<AtomHeader>>,
    
    pub weight_transform: AtomWeightTransform,
    pub traversal: TranversalEngine
}

#[cfg(test)]
mod tests {
    use super::*;
    use pathmap::{
        PathMap,
    };

    #[test]
    fn it_works() {
        let map = WeightedAtomSweep::with_process(
            PathMap::<AtomHeader>::new().into_zipper_head([]), // The main map usable by MORK
            AtomWeightTransform { ... },  // Weighted 
            TranversalEngine { 
                traverse: Arc::new(|zipper: WriteZipperTracked<AtomHeader>| {
                    // execute AtomWeightTransform.transform
                }),
                transform: Arc::new(|zipper: WriteZipperTracked<AtomHeader>| {
                    // What 
                }),
            }, // Traversal engine
        );

        // runs on mork
        let map = PathMap::<AtomHeader>::new().into_zipper_head([]);

        let sweep = WeightedAtomSweep::with_process(  // this spawns a background thread.
            PathMap::<AtomHeader>::new().into_zipper_head([]), // The main map usable by MORK
            AtomWeightTransform { ... },  // logic and settings for weighted updating. this can have default implementations. 
                // Holds the necessary logic 
                TranversalEngine { 
                    traverse: Arc::new(
                        |zipper: WriteZipperTracked<AtomHeader>|  // takes a zipper focused on the
                                                                // atom being visited 
                    {
                    }),
                    transform: Arc::new(
                        |zipper: WriteZipperTracked<AtomHeader>|  // 
                    {
                        // What 
                    }),
                }, // Traversal engine
        );

        
    }
}
