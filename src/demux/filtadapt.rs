use crate::io::Pair;

pub struct FiltAdapt{
    adapter: String
}
impl FiltAdapt {
    pub fn new(adapter: String) -> Self {
        Self { adapter }
    }
    pub fn contains_adapter(&self, pair: &Pair) -> bool {
        pair.r2_seq().starts_with(&self.adapter)
    }
    pub fn strip_adapter(&self, pair: &mut Pair) {
        if self.contains_adapter(pair) {
           pair.trim_adapter(&self.adapter);
        }
    }
}
