use std::collections::HashMap;

use gdemux::demux::{BUS, AssignedBarcode};
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
#[test]
/// Tests instantiation
fn build(){
    let _ab = AssignedBarcode::new("ACTG", "class_a");
}

#[test]
/// Tests instantiation from BUS conversion
fn build_from_bus(){
    let bus = build_const_bus_a();
    let _ab = AssignedBarcode::from_bus(&bus);
}

#[test]
/// Tests equality conditions between assignments
fn equality(){
    let ab1 = AssignedBarcode::new("ACTG", "class_a");
    let ab2 = AssignedBarcode::new("ACTG", "class_b");
    let ab3 = AssignedBarcode::new("GTCA", "class_a");
    let ab4 = AssignedBarcode::new("GTCA", "class_b");

    assert_eq!(ab1, ab1);
    assert_ne!(ab1, ab2);
    assert_ne!(ab1, ab3);
    assert_ne!(ab1, ab4);
}

#[test]
/// Tests hashing qualities between assignments
fn hashing(){
    let ab1 = AssignedBarcode::new("ACTG", "class_a");
    let ab2 = AssignedBarcode::new("ACTG", "class_b");
    let ab3 = AssignedBarcode::new("GTCA", "class_a");
    let ab4 = AssignedBarcode::new("GTCA", "class_b");

    let mut map = HashMap::new();
    let counter = map.entry(&ab1).or_insert(0);
    *counter += 1;
    let counter = map.entry(&ab2).or_insert(0);
    *counter += 1;
    let counter = map.entry(&ab3).or_insert(0);
    *counter += 1;
    let counter = map.entry(&ab4).or_insert(0);
    *counter += 1;

    assert_eq!(*map.get(&ab1).unwrap(), 1);
    assert_eq!(*map.get(&ab2).unwrap(), 1);
    assert_eq!(*map.get(&ab3).unwrap(), 1);
    assert_eq!(*map.get(&ab4).unwrap(), 1);
}
