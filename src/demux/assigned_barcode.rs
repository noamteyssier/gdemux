use super::BUS;
use std::hash::{Hash, Hasher};

#[derive(Debug, Eq)]
pub struct AssignedBarcode {
    barcode: String,
    assignment: String
}
impl AssignedBarcode {

    pub fn new(barcode: &str, assignment: &str) -> Self {
        Self { 
            barcode: barcode.to_string(), 
            assignment: assignment.to_string() 
        }
    }

    pub fn from_bus(bus: &BUS) -> Self {
        Self::new(bus.barcode(), bus.assignment())
    }

    pub fn barcode(&self) -> &str {
        &self.barcode
    }

    pub fn assignment(&self) -> &str {
        &self.assignment
    }

}
impl Hash for AssignedBarcode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.barcode.hash(state);
        self.assignment.hash(state);
    }
}
impl PartialEq for AssignedBarcode {
    fn eq(&self, other: &AssignedBarcode) -> bool {
        (self.barcode == other.barcode()) &
            (self.assignment == other.assignment())
    }
}
