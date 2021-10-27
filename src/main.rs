mod io;
mod demux;
use demux::{FiltAdapt, Whitelist, BUS, Demux};
use io::PairReader;
use clap::{Arg, App};

fn get_args() -> App<'static, 'static> {
    App::new("gDemux")
        .version("0.1")
        .author("Noam Teyssier <Noam.Teyssier@ucsf.edu>")
        .about("Parses a pair of Raw FASTQ files and calculates the number of UMIs that belong to each barcode-guide interaction")
        .arg(Arg::with_name("INPUT_R1")
            .short("i")
            .long("r1")
            .help("Sets the input R1 fastq file to use (*.fastq, *.fq, *.fastq.gz, *.fq.gz)")
            .required(true)
            .min_values(1))
        .arg(Arg::with_name("INPUT_R2")
            .short("I")
            .long("r2")
            .help("Sets the input R2 fastq file to use (*.fastq, *.fq, *.fastq.gz, *.fq.gz)")
            .required(true)
            .min_values(1))
        .arg(Arg::with_name("barcode_whitelist")
            .short("c")
            .long("barcode_whitelist")
            .help("Sets the input barcode whitelist to screen R1 against")
            .required(true)
            .min_values(1))
        .arg(Arg::with_name("guide_whitelist")
            .short("g")
            .long("guide_whitelist")
            .help("Sets the input guide whitelist to map R2 against (can be .txt, .tsv)")
            .required(true)
            .min_values(1))
        .arg(Arg::with_name("SIZE_UMI")
                .short("u")
                .long("umi_size")
                .help("Sets the size of the UMI to use (default=12)")
                .required(false)
                .takes_value(true)
                .default_value("12"))
        .arg(Arg::with_name("ADAPTER")
            .short("a")
            .long("adapter")
            .help("Sets the adapter sequence to match R2 with")
            .required(false)
            .takes_value(true)
            .default_value("AGTATCCCTTGGAGAACCACCTTG"))
}

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

    let barcode_whitelist = Whitelist::from_gzip_file(barcode_filename);
    let guide_whitelist = Whitelist::from_table(table_filename, '\t');

    let b_size = barcode_whitelist.size();
    let s_size = guide_whitelist.size();

    let pairs = PairReader::from_gzip(fn1, fn2);
    let filt_adapt = FiltAdapt::new(adapter.to_string());
    let mut demux = Demux::new();

    pairs.into_iter()
        
        // Read Pairs
        .map(|x| x.expect("Error reading pairs"))
        
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


    demux.pretty_print();
}
