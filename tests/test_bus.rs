use gdemux::demux::BUS;
use gdemux::io::Pair;
use bio::io::FastqRecord;

fn build_const_bus_a() -> BUS {
    let mut fq1 = FastqRecord::new();
    let mut fq2 = FastqRecord::new();
    fq1.assign_seq("ACTGACTG");
    fq2.assign_seq("GTCAGTCA");

    let p = Pair::new(fq1, fq2);
    
    let mut bus = BUS::from_pair(p, 4, 4, 8).unwrap();
    bus.assign("class_a");
    bus
}

/// Alternative to Default
/// Different Barcode
/// Different UMI
/// Different Class
fn build_const_bus_b() -> BUS {
    let mut fq1 = FastqRecord::new();
    let mut fq2 = FastqRecord::new();
    fq1.assign_seq("ACTTTCTG");
    fq2.assign_seq("GTCTTTCA");

    let p = Pair::new(fq1, fq2);

    let mut bus = BUS::from_pair(p, 4, 4, 8).unwrap();
    bus.assign("class_b");
    bus
}

/// Alternative to Default 
/// Same Barcode
/// Different UMI
/// Same Class
fn build_const_bus_c() -> BUS {
    let mut fq1 = FastqRecord::new();
    let mut fq2 = FastqRecord::new();
    fq1.assign_seq("ACTGCCCC");
    fq2.assign_seq("GTCAGTCA");

    let p = Pair::new(fq1, fq2);
    
    let mut bus = BUS::from_pair(p, 4, 4, 8).unwrap();
    bus.assign("class_a");
    bus
}

/// Alternative to Default 
/// Same Barcode
/// Same UMI
/// Different Class
fn build_const_bus_d() -> BUS {
    let mut fq1 = FastqRecord::new();
    let mut fq2 = FastqRecord::new();
    fq1.assign_seq("ACTGACTG");
    fq2.assign_seq("GTCAGTCA");

    let p = Pair::new(fq1, fq2);
    
    let mut bus = BUS::from_pair(p, 4, 4, 8).unwrap();
    bus.assign("class_b");
    bus
}

/// Alternative to Default 
/// Different Barcode
/// Same UMI
/// Same Class
fn build_const_bus_e() -> BUS {
    let mut fq1 = FastqRecord::new();
    let mut fq2 = FastqRecord::new();
    fq1.assign_seq("GGGGACTG");
    fq2.assign_seq("GTCAGTCA");

    let p = Pair::new(fq1, fq2);
    
    let mut bus = BUS::from_pair(p, 4, 4, 8).unwrap();
    bus.assign("class_a");
    bus
}

#[test]
fn test_bus_init(){
    let bus = build_const_bus_a();
    assert_eq!(bus.barcode(), "ACTG");
    assert_eq!(bus.umi(), "ACTG");
    assert_eq!(bus.seq(), "GTCAGTCA");
}

#[test]
fn test_bus_eq(){
    let bus_a = build_const_bus_a();
    assert_eq!(bus_a, bus_a);
}

#[test]
fn test_bus_ne_umi(){
    let bus_a = build_const_bus_a();
    let bus_c = build_const_bus_c();
    assert_ne!(bus_a, bus_c);
}

#[test]
fn test_bus_ne_barcode(){
    let bus_a = build_const_bus_a();
    let bus_e = build_const_bus_e();
    assert_ne!(bus_a, bus_e);
}

#[test]
fn test_bus_ne_class(){
    let bus_a = build_const_bus_a();
    let bus_d = build_const_bus_d();
    assert_ne!(bus_a, bus_d);
}

#[test]
fn test_bus_ne_all(){
    let bus_a = build_const_bus_a();
    let bus_b = build_const_bus_b();
    assert_ne!(bus_a, bus_b);
}
