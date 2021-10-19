use std::io::Error;

use bio::io::{Buf, BufGz, FastqRead, FastqRecord, Reader, Records};

pub struct Pair {
    r1: FastqRecord,
    r2: FastqRecord
}
impl Pair {
    pub fn new(
        r1: Result<FastqRecord, Error>, 
        r2: Result<FastqRecord, Error>) -> Self 
    {
        Self{
            r1: r1.expect("Error: Malformed R1 Record"), 
            r2: r2.expect("Error: Malformed R2 Record")
        }
    }

    pub fn r1_seq(&self) -> &str {
        self.r1.seq()
    }

    pub fn r2_seq(&self) -> &str {
        self.r2.seq()
    }

}

pub struct PairReader <F> {
    r1 : Records<F>,
    r2 : Records<F>
}
impl <F> PairReader <F>
where
    F: FastqRead
{
    pub fn new(r1: Records<F>, r2: Records<F>) -> Self {
        Self { r1, r2 }
    }
}

impl PairReader<Reader<Buf>> {
    pub fn from_file(path1: &str, path2: &str) -> Self {
        let r1 = Records::from_file(path1);
        let r2 = Records::from_file(path2);
        Self { r1, r2 }

    }
}
impl PairReader<Reader<BufGz>> {
    pub fn from_gzip(path1: &str, path2: &str) -> Self {
        let r1 = Records::from_gzip(path1);
        let r2 = Records::from_gzip(path2);
        Self { r1, r2 }

    }
}
impl <R> Iterator for PairReader<R> 
where
    R: FastqRead
{
    type Item = Result<Pair, Error>;

    fn next(&mut self) -> Option<Result<Pair, Error>> {
        let r1_rec = self.r1.next();
        let r2_rec = self.r2.next();
        if r1_rec.is_some() & r2_rec.is_some() {
            let pair = Pair::new(
                r1_rec.unwrap(),
                r2_rec.unwrap()
                );
            Some(Ok(pair))
        }
        else {
            None
        }
    }
}

