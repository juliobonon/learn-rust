use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let file = File::open(&args[1])?;
    let reader = BufReader::new(file);

   
    for line in reader.lines() {
        println!("{}", line?);
    
    }

    Ok(())

}
