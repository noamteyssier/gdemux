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
