use std::io::BufRead;

#[derive(Debug)]
enum Error {
    Open(std::io::Error),
    Read(std::io::Error),
}

fn read_file() -> Result<(), Error> {
    let file = std::fs::File::open("out.txt").map_err( Error::Open)?;
    let reader = std::io::BufReader::new(file);
    for line in reader.lines() {
        let line = line.map_err(Error::Read)?;
        println!("{}", line);
    }
    Ok(())
}
fn main() -> Result<(), Error> {
    read_file()
}
