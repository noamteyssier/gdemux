mod io;
mod demux;
mod utils;
use std::time::Instant;
use indicatif::ProgressBar;

use demux::{FiltAdapt, BUS, Demux};
use io::PairReader;
use utils::{get_args, load_barcodes, load_guides};

/// This is a library to perform Barcode-UMI-Sequence (BUS) Demultiplexing against two whitelists.
/// Guide will be used interchangeably with sequence (BUG == BUS)
/// 
///
/// Sequence Layout:
///     R1:
///         <BARCODE><UMI>[ACTG]?+
///     R2:
///         <ADAPTER?><GUIDE>[ACTG]?+
///
/// The following will be performed:
///     - Only reads with an adapter sequence will be considered
///     - Only reads with a barcode found in the whitelist will be considered
///     - Adapter sequence will be trimmed
///     - A BUS will be created from the pair
///     - A guide class will be assigned (either as the guide alias or the guide sequence)
///     - The BUS will be counted
///
/// The following will be calculated:
///     - The number of UMIs of each Barcode-Guide
fn main() {
    let args = get_args().get_matches();
    let fn1 = args.value_of("INPUT_R1")
            .expect("Error: Unable to access input R1");
    let fn2 = args.value_of("INPUT_R2")
            .expect("Error: Unable to access input R2");
    let barcode_filename = args.value_of("barcode_whitelist")
            .expect("Error: Unable to access barcode whitelist");
    let table_filename = args.value_of("guide_whitelist")
            .expect("Error: Unable to access guide whitelist");
    let adapter = args.value_of("ADAPTER")
            .expect("Error: Unable to access adapter");
    let u_size = args.value_of("SIZE_UMI")
            .expect("Error: Unable to access UMI size from arguments")
            .parse::<usize>()
            .expect("Error: Unable to parse given UMI size to int");

    let barcode_whitelist = load_barcodes(barcode_filename)
        .expect("Error: Unable to load barcode whitelist");
    let guide_whitelist = load_guides(table_filename).
        expect("Error: Unable to load guide whitelist/map");

    let b_size = barcode_whitelist.size();
    let s_size = guide_whitelist.size();

    let pairs = PairReader::from_gzip(fn1, fn2);
    let filt_adapt = FiltAdapt::new(adapter.to_string());
    let mut demux = Demux::new();

    let pbr = ProgressBar::new_spinner();
    pbr.enable_steady_tick(100);
    let start_time = Instant::now();
    let mut num_total = 0;

    pairs.into_iter()
        
        // Read Pairs
        .map(|x| {
            num_total += 1;
            if num_total % 10000 == 0 {
                pbr.set_message(&format!(
                    "Processing... {} records // {:.2} sec elapsed", 
                    num_total, 
                    start_time.elapsed().as_secs_f32()
                    ));
            }
            x.expect("Error reading pairs")
        })
        
        // Filter for R2 beginning with adapter
        .filter(|x| filt_adapt.contains_adapter(&x))

        // Filter for R1 in whitelist
        .filter(|x| barcode_whitelist.contains(x.r1_seq()))
        
        // Trim Adapter sequence
        .map(|mut x| {filt_adapt.strip_adapter(&mut x); x})

        // Convert to BUS
        .filter_map(|x| BUS::from_pair(x, b_size, u_size, s_size))

        // Assign each BUS to a guide sequence (if matching)
        .filter_map(|mut x| {
            match guide_whitelist.match_sequence(x.seq()) {
                Some(s) => {
                    x.assign(&s);
                    Some(x)
                }
                None => None
            }
        })

        // Insert BUS to growing counts
        .for_each(|x| demux.insert_bus(x));
        
    pbr.set_message(&format!(
        "Processing... {} records // {:.2} sec elapsed",
        num_total,
        start_time.elapsed().as_secs_f32()
        ));
    pbr.finish();


    demux.pretty_print();
}
