use gdemux::demux::{Demux, BUS};
use gdemux::io::Pair;
use bio::io::FastqRecord;

/// Default BUS Implementation
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
/// Tests instantiation of Demux
fn test_demux_init() {
    let _demux = Demux::new();
}

#[test]
/// tests whether a bus can be inserted to hashmap
fn test_demux_insert() {
    let mut demux = Demux::new();
    let bus = build_const_bus_a();
    demux.insert_bus(bus);
}

#[test]
/// tests whether the contains method is correctly identifying a BUS
fn test_demux_contains() {
    let mut demux = Demux::new();
    let bus_a = build_const_bus_a();
    let bus_b = build_const_bus_a();

    assert_eq!(bus_a, bus_b);
    demux.insert_bus(bus_a);
    assert!(demux.contains_bus(&bus_b))
}

#[test]
/// tests whether the correct number of BUS items have been inserted
fn test_demux_counter() {
    let mut demux = Demux::new();
    let const_bus_a = build_const_bus_a();

    (0..20).for_each(|_| {
        let b = build_const_bus_a();
        demux.insert_bus(b);
    });
    assert_eq!(*demux.occurences_bus(&const_bus_a).unwrap(), 20);
}

#[test]
/// tests whether the correct number of BUS items have been inserted
/// when there are alternative classes inserted
fn test_demux_counter_alt_diff() {
    let mut demux = Demux::new();
    let const_bus_a = build_const_bus_a();
    let const_bus_b = build_const_bus_b();
    let const_bus_c = build_const_bus_c();
    let const_bus_d = build_const_bus_d();
    let const_bus_e = build_const_bus_e();

    (0..20).for_each(|i| {
        let b = match i % 5 {
            0 => build_const_bus_a(),
            1 => build_const_bus_b(),
            2 => build_const_bus_c(),
            3 => build_const_bus_d(),
            _ => build_const_bus_e()
        };
        demux.insert_bus(b);
    });
    assert_eq!(*demux.occurences_bus(&const_bus_a).unwrap(), 4);
    assert_eq!(*demux.occurences_bus(&const_bus_b).unwrap(), 4);
    assert_eq!(*demux.occurences_bus(&const_bus_c).unwrap(), 4);
    assert_eq!(*demux.occurences_bus(&const_bus_d).unwrap(), 4);
    assert_eq!(*demux.occurences_bus(&const_bus_e).unwrap(), 4);
}

