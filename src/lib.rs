use needletail::{parse_fastx_file,};
use needletail::parser::{SequenceRecord, LineEnding, write_fastq};
use flate2::Compression;
use flate2::write::GzEncoder;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::process::exit;


pub fn foo(path: &str, outpath: &str, minqual: f64, minlen: usize, maxlen: usize, offset: u8) {
    let mut reader = parse_fastx_file(path).expect("Failed to open parser");
    let mut writer = GzEncoder::new(
        BufWriter::new(
        match OpenOptions::new().write(true).create_new(true).open(outpath) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Error opening new file: {}", e);
                exit(1);
            }
        }
        ), Compression::new(3));
    
    while let Some(maybe_record) = reader.next() {
        let record = maybe_record.expect("Failed to parse record");
        if passes(&record, minqual, minlen, maxlen, offset) {
            write_fastq_record(&mut writer, &record)
        }
    }
}

fn passes(rec: &SequenceRecord, min_qual: f64, 
            min_len: usize, max_len: usize, offset: u8) -> bool {
    if mean_phred_qual(rec, offset) < min_qual { return false }
    if rec.seq().len() < min_len || rec.seq().len() > max_len { return false }
    true
}

fn phred_to_prob(phred: u8, offset: u8) -> f64 {
    (10.0_f64).powf((phred - offset) as f64 / -10.0)
}

fn prob_to_phred(prob: f64) -> f64 {
    prob.log10() * -10.0_f64
}

fn mean_phred_qual(rec: &SequenceRecord, offset: u8) -> f64 {
    let mut n: f64 = 0.0;
    let mut prob: f64 = 0.0;
    for phred in rec.qual().unwrap() {
        n += 1.0;
        prob += phred_to_prob(*phred, offset);
    }
    prob_to_phred(prob / n)
}
    
fn write_fastq_record(writer: &mut impl Write, rec: &SequenceRecord) {
    write_fastq(rec.id(), &rec.seq(), rec.qual(),
                writer, LineEnding::Unix).expect("Failed to write record.")
}
