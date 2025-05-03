use std::path::Path;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    BadLineArgument(usize),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
    
}

fn read_nth_line(path: &Path, n: usize) -> Result<String, Error> {
    

    if n < 1 {
        return Err(Error::BadLineArgument(n));
    }
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    let file = File::open(path)?;

    let mut reader_lines = BufReader::new(file).lines();

    reader_lines.nth(n-1).map(|result| result.map_err(|err| err.into())).unwrap_or_else(|| Err(Error::BadLineArgument(n)))
}

fn main() {
    let path = Path::new("Cargo.toml");

    println!("The 4th line of cargo.toml is:{}", read_nth_line(path, 4).unwrap());
}



