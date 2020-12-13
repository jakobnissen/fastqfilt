use needletail::parse_fastx_file;

pub fn foo(path: &str) -> anyhow::Result<()> {
    let mut reader = parse_fastx_file(path)?;
    let mut n: usize = 0;
    let mut slen: usize = 0;
    while let Some(maybe_record) = reader.next() {
        let record = maybe_record?;
        n += 1;
        slen += record.seq().len();
    }
    println!("{} {}", n, slen);
    Ok(())
}
