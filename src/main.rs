mod io;
use io::PairReader;

fn main() {
    let fn1 = "data/test_R1.fastq.gz";
    let fn2 = "data/test_R2.fastq.gz";
    let pairs = PairReader::from_gzip(fn1, fn2);

    pairs.into_iter()
        .map(|x| x.expect(""))
        .for_each(|x| println!("{} {}", x.r1_seq(), x.r2_seq()));
}
