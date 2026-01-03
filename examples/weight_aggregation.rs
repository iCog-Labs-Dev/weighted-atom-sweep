use weighted_atom_sweep::{AtomHeader, WeightedValue, WeightedMap};

#[derive(Debug, Clone, PartialEq)]
pub struct SimpleWeight {
    value: f64,
}

impl AtomHeader for SimpleWeight {
    fn add(&self, other: &Self) -> Self {
        SimpleWeight { value: self.value + other.value }
    }
}

impl Default for SimpleWeight {
    fn default() -> Self {
        Self { value: 0.0 }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Weight Aggregation Demo ===");
    
    // Create a weighted map
    let map = pathmap::PathMap::new();
    let weighted_map = WeightedMap::new(map.into_zipper_head(&[]));
    
    // Insert initial values
    let leaf1_val = WeightedValue {
        val: SimpleWeight { value: 2.0 },
        child_agg_w: SimpleWeight { value: 0.0 },
    };
    
    let leaf2_val = WeightedValue {
        val: SimpleWeight { value: 3.0 },
        child_agg_w: SimpleWeight { value: 0.0 },
    };
    
    // First create the parent node, then the children
    weighted_map.set_val(b"child1", 
        WeightedValue { val: SimpleWeight { value: 0.0 }, child_agg_w: SimpleWeight { value: 0.0 } },
        );
    weighted_map.set_val(b"child1leaf1", leaf1_val);
    weighted_map.set_val(b"child1leaf2", leaf2_val.clone());
    weighted_map.set_val(b"child1leaf3", leaf2_val);
    
    println!("\nUpdating leaf1 from 2.0 to 2.5...");
    weighted_map.set_weighted_val(
        b"child1leaf1",
        SimpleWeight { value: 2.5 },
    )?;

    // Verify the update
    if let Some(updated) = weighted_map.get_val(b"child1leaf1") {
        println!("✓ Updated child1leaf1: val = {}, child_agg_w = {}", 
                 updated.val.value, updated.child_agg_w.value);
    }
    if let Some(updated) = weighted_map.get_val(b"child1leaf") {
        println!("✓ Updated child1leaf: val = {}, child_agg_w = {}", 
                 updated.val.value, updated.child_agg_w.value);
    } else {
        println!("No val found at child1leaf");
    }
    if let Some(updated) = weighted_map.get_val(&[]) {
        println!("✓ Updated root: val = {}, child_agg_w = {}", 
                 updated.val.value, updated.child_agg_w.value);
    } else {
        println!("No val found at []");
    }
    
    println!("\n=== Demo completed successfully! ===");
    Ok(())
}

