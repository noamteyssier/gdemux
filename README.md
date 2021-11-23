# gdemux
Guide Demultiplexing for CROPseq Data

# Installation
```bash
git clone https://github.com/noamteyssier/gdemux
cd gdemux
cargo install --path .
```

# Usage
```bash

gdemux\
	-i ${base}_R1.fastq.gz \
	-I ${base}_R2.fastq.gz \
	-c ${barcodes} \
	-g ${guides}
```

# File Formats
## Barcodes
The whitelist barcodes are expected to be a plain text formatted file where each line is a specific barcode that is expected to be in the dataset. An example of this whitelist is the 10xv3 or 10xv2 lists. These can be provided as an argument to gdemux as either plaintext or in a gzip compressed format. 

The 10Xv3 whitelist can be found here: https://github.com/10XGenomics/cellranger/blob/a83c753ce641db6409a59ad817328354fbe7187e/lib/python/cellranger/barcodes/translation/3M-february-2018.txt.gz

There can not be duplicates in this whitelist, and the program will quit if it finds one. 

### Example
```
AAACCCAAGAAACACT
AAACCCAAGAAACCAT
AAACCCAAGAAACCCA
AAACCCAAGAAACCCG
```

## Guides
The guide whitelist can be provided in two formats: exactly as the cell barcodes (i.e. each line is a sequence to match) or as a two column table (tab separated) where the first column is the guide sequence and the second is the alias of that sequence (the name of the guide). These can be provided either as plain text or in a gzip compressed format. 

There can not be duplicates in this whitelist, and the program will quit if it finds one. 

### Example
```
TTGGCAGGCCCGTTTGCTTACGAGTTTAAGAGC	NTC_3083
TTGGAGTCTGGGGAGGACATTGTGTTTAAGAGC	NTC_5404
TTGGACCCGAGACTGCTTCCCGGGTTTAAGAGC	NTC_5406
TTGGTACCACCCAAACGATAACGGTTTAAGAGC	NTC_5408
```
