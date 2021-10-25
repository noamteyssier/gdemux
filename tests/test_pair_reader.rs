use bio::io::Records;
use gdemux::io::PairReader;

static FN1: &str = "tests/data/test_R1.fq.gz";
static FN2: &str = "tests/data/test_R2.fq.gz";

#[test]
fn test_pair_reader_from_gzip(){
    let pairs = PairReader::from_gzip(FN1, FN2);

    let mut count = 0;
    pairs.into_iter()
        .map(|x| x.expect(""))
        .for_each(|_| count += 1);
    assert_eq!(count, 2500);
}

#[test]
fn test_pair_reader_from_file(){
    let pairs = PairReader::from_gzip(FN1, FN2);

    let mut count = 0;
    pairs.into_iter()
        .map(|x| x.expect(""))
        .for_each(|_| count += 1);
    assert_eq!(count, 2500);
}

#[test]
fn test_pair_reader_from_gzip_init_records(){
    let r1 = Records::from_gzip(FN1);
    let r2 = Records::from_gzip(FN2);
    let pairs = PairReader::new(r1, r2);

    let mut count = 0;
    pairs.into_iter()
        .map(|x| x.expect(""))
        .for_each(|_| count += 1);
    assert_eq!(count, 2500);
}

#[test]
fn test_pair_reader_from_file_init_records(){
    let r1 = Records::from_gzip(FN1);
    let r2 = Records::from_gzip(FN2);
    let pairs = PairReader::new(r1, r2);

    let mut count = 0;
    pairs.into_iter()
        .map(|x| x.expect(""))
        .for_each(|_| count += 1);
    assert_eq!(count, 2500);
}
