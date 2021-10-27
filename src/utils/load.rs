use crate::{demux::Whitelist};


pub fn load_barcodes(filename: &str) -> Option<Whitelist> {
    if filename.contains(".txt.gz") {
        Some(Whitelist::from_gzip_file(filename))
    }
    else if filename.contains(".txt") {
        Some(Whitelist::from_file(filename))
    }
    else {
        None
    }
}

pub fn load_guides(filename: &str) -> Option<Whitelist> {
    if filename.contains(".txt.gz") {
        Some(Whitelist::from_gzip_file(filename))
    }
    else if filename.contains(".txt") {
        Some(Whitelist::from_file(filename))
    }
    else if filename.contains(".tsv") {
        Some(Whitelist::from_table(filename, '\t'))
    }
    else if filename.contains(".csv") {
        Some(Whitelist::from_table(filename, ','))
    }
    else if filename.contains(".tab") {
        Some(Whitelist::from_table(filename, '\t'))
    }
    else {
        None
    }
}
