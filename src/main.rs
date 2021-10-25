mod io;
mod demux;
use demux::{FiltAdapt, Whitelist};
use io::PairReader;


fn main() {
    let barcode_filename = "data/cell_barcodes_10xv3.txt.gz";
    let table_filename = "data/cropseq_guides.tab";

    let fn1 = "data/test_R1.fq.gz";
    let fn2 = "data/test_R2.fq.gz";

    let adapter = "AGTATCCCTTGGAGAACCACCTTG";

    println!("{}", "Loading Barcodes");
    let barcode_whitelist = Whitelist::from_gzip_file(barcode_filename);

    println!("{}", "Loading Guides");
    let mut guide_whitelist = Whitelist::from_table(table_filename, '\t');

    let pairs = PairReader::from_gzip(fn1, fn2);
    let filt_adapt = FiltAdapt::new(adapter.to_string());

    pairs.into_iter()
        
        // Read Pairs
        .map(|x| x.expect("Error reading pairs"))
        
        // Filter for R2 beginning with adapter
        .filter(|x| filt_adapt.contains_adapter(&x))

        // Filter for R1 in whitelist
        .filter(|x| barcode_whitelist.contains(x.r1_seq()))
        
        // Trim Adapter sequence
        .map(|mut x| {filt_adapt.strip_adapter(&mut x); x})

        // Increment Guide Counts 
        .for_each(|x| guide_whitelist.increment_counts(x.r2_seq()));

    guide_whitelist.pretty_print();
}
