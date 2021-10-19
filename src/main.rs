mod io;
mod demux;
use demux::FiltAdapt;
use io::PairReader;


fn main() {
    let fn1 = "data/test_R1.fastq.gz";
    let fn2 = "data/test_R2.fastq.gz";
    let adapter = "TTCCAGCTTAGCTCTTAAAC";
    let pairs = PairReader::from_gzip(fn1, fn2);

    let filt_adapt = FiltAdapt::new(adapter.to_string());

    pairs.into_iter()
        
        // Read Pairs
        .map(|x| x.expect("Error reading pairs"))
        
        // Filter for R2 beginning with adapter
        .filter(|x| filt_adapt.contains_adapter(&x))
        
        // Trim Adapter sequence
        .for_each(|mut x| filt_adapt.strip_adapter(&mut x));
}
