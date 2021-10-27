use std::collections::HashMap;
use super::{BUS, AssignedBarcode};

pub struct Demux {
    busmap: HashMap<BUS, usize>,
    assigned_map: HashMap<AssignedBarcode, usize>
}
impl Demux{
    pub fn new() -> Self {
        Self {
            busmap: HashMap::new(),
            assigned_map: HashMap::new()
        }
    }

    pub fn insert_assignment(&mut self, bus: &BUS) {
        let assignment = AssignedBarcode::from_bus(bus);  

        // Increments Counter for Assignment
        let counter = self.assigned_map.entry(assignment).or_insert(0);
        *counter += 1;
    }

    pub fn insert_bus(&mut self, bus: BUS) {
        if !self.contains_bus(&bus) {
            self.insert_assignment(&bus);
        }

        // Increments Counter for BUS
        let counter = self.busmap.entry(bus).or_insert(0);
        *counter += 1;
    }

    pub fn contains_bus(&self, bus: &BUS) -> bool {
        self.busmap.contains_key(bus)
    }

    pub fn contains_assignment(&self, ab: &AssignedBarcode) -> bool {
        self.assigned_map.contains_key(ab)
    }

    pub fn occurences_bus(&self, bus: &BUS) -> Option<&usize> {
        if self.contains_bus(bus) {
            Some(self.busmap.get(bus).unwrap())
        }
        else {
            None    
        }
    }

    pub fn occurences_assignment(&self, ab: &AssignedBarcode) -> Option<&usize>{
        if self.contains_assignment(ab) {
            Some(self.assigned_map.get(ab).unwrap())
        }
        else {
            None
        }
    }

    pub fn pretty_print(&self) {
        for (key, value) in &self.assigned_map {
            println!("{}\t{}\t{}", key.barcode(), key.assignment(), value);
        }
    }
}
