use gdemux::demux::Whitelist;

static WL1: &str = "tests/data/guide_list.tab";
static WL2: &str = "tests/data/test_whitelist.txt.gz";
static WL3: &str = "tests/data/test_whitelist.txt";
static WL4: &str = "tests/data/test_whitelist_incorrect_size.txt";
static WL5: &str = "tests/data/test_whitelist_duplicate_seq.txt";

#[test]
fn whitelist_tab(){
    let wl = Whitelist::from_table(WL1, '\t');
    assert_eq!(wl.size(), 33);
    assert_eq!(wl.members(), 4)
}

#[test]
fn whitelist_gzip(){
    let wl = Whitelist::from_gzip_file(WL2);
    assert_eq!(wl.size(), 16);
    assert_eq!(wl.members(), 4)
}

#[test]
fn whitelist_txt(){
    let wl = Whitelist::from_file(WL3);
    assert_eq!(wl.size(), 16);
    assert_eq!(wl.members(), 4)
}

#[test]
#[should_panic]
fn incorrect_size(){
    let _wl = Whitelist::from_file(WL4);
}

#[test]
#[should_panic]
fn duplicate_seq(){
    let _wl = Whitelist::from_file(WL5);
}
