use std::{error::Error};

use csv;

fn read_from_file(path: &str) -> Result<(), Box<dyn Error>>{
    let mut reader = csv::Reader::from_path(path)?;

    for result in reader.records() {
        let record = result?;
        println!("{:?}", record);
    }

    Ok(())
}

fn main() {
    let args: Vec<_> = std::env::args().collect();

    if args.len() < 2 {
        panic!("!!! Vous devez rentrer un nom de fichier csv !!!");
    }

    // c'est bon, un fichier csv est passé en paramètre
    let fname = std::path::Path::new(&*args[1]);

    if let Err(e) = read_from_file(&fname.display().to_string()){
        eprintln!("{}", e);
    }
}
