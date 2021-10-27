use crate::io::Pair;
use std::hash::{Hash, Hasher};

#[derive(Eq, Debug)]
pub struct BUS {
    barcode: String,
    umi: String,
    seq: String,
    assignment: String
}
impl BUS {
    pub fn from_pair(
        p: Pair,
        b_size: usize,
        u_size: usize,
        s_size: usize) -> Option<Self>
    {
        if p.r1_seq().len() < b_size + u_size {
            None
        }
        else if p.r2_seq().len() < s_size {
            None
        }
        else {
            Some(
                Self {
                    barcode: p.r1_seq()[0..b_size].to_string(),
                    umi: p.r1_seq()[b_size..b_size+u_size].to_string(),
                    seq: p.r2_seq()[0..s_size].to_string(),
                    assignment: String::new()
                }
            )
        }
    }

    pub fn barcode(&self) -> &str {
        &self.barcode
    }

    pub fn umi(&self) -> &str {
        &self.umi
    }

    pub fn seq(&self) -> &str {
        &self.seq
    }

    pub fn assignment(&self) -> &str {
        &self.assignment
    }

    pub fn assign(&mut self, s: &str) {
        self.assignment = s.to_string();
    }
}
impl Hash for BUS {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.barcode.hash(state);
        self.umi.hash(state);
        self.assignment.hash(state);
    }
}
impl PartialEq for BUS {
    fn eq(&self, other: &BUS) -> bool {
        (self.barcode == other.barcode()) &
            (self.umi == other.umi()) &
            (self.assignment == other.assignment())
    }
}

