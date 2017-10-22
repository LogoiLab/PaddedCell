extern crate base64;
#[macro_use]
extern crate clap;
use clap::{Arg, App};
#[macro_use]
extern crate serde_derive;
extern crate toml;

use std::error::Error;
use std::fs::File;
use std::io::Bytes;
use std::io::prelude::*;
use std::path::Path;

use base64::{encode, decode};

#[derive(Serialize)]
#[derive(Deserialize)]
struct Database {
    samples: Vec<Sample>,
    size: u64,
}

#[derive(Serialize)]
#[derive(Deserialize)]
struct Sample {
    filename: String,
    data: String,
    md5: String,
    sha256: String,
}

fn compute_md5() {
    unimplemented!()
}

fn compute_sha256() {
    unimplemented!()
}

fn make_safe(file: Bytes<File>) -> String {
    let mut byte_vec: Vec<u8> = vec!();
    for byte in file {
        byte_vec.push(byte.expect("Invalid byte."));
    }
    let byte_arr = byte_vec.as_slice();

    encode(byte_arr)
}

fn make_live(b64: &String) -> Vec<u8> {
    decode(&b64).expect("Failed to decode binary.")
}

fn read_bytes(path: &str) -> Bytes<File> {
    let path = Path::new(path);
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    file.bytes()
}

fn load_db(db: Database) -> Database {
    let path = Path::new("mal_database.toml");
    let display = path.display();

    let mut db_file = match File::open(&path) {
        Err(why) => {
            File::create("mal_database.toml").expect("Failed to create file.").write(toml::to_string(&db).unwrap().as_bytes());
            panic!("Failed to read database. Creating.");
        },
        Ok(file) => file,
    };
    let mut db_strings: String = String::new();
    db_file.read_to_string(&mut db_strings).unwrap();
    toml::from_str(&db_strings.as_str()).expect("Failed to parse database.")
}

fn save_db(db: Database) {
    File::open("mal_database.toml").expect("Failed to create file.").write(toml::to_string(&db).unwrap().as_bytes());
}

fn main() {
    let mut db: Database = Database{samples: vec!(), size: 0};
    db = load_db(db);
    let matches = App::new(crate_name!()).version(crate_version!()).author(crate_authors!()).about(crate_description!()).arg(Arg::with_name("import").short("i").long("import").value_name("IMPORT_FILE").help("Imports a sample into the database").takes_value(true).required(false)).arg(Arg::with_name("export").short("x").long("export").value_name("EXPORT_FILE").help("Exports the given sample from the database").takes_value(true).required(false)).arg(Arg::with_name("list").short("l").long("list").help("Lists all samples in database").takes_value(false).required(false)).get_matches();

    match matches.occurrences_of("IMPORT_FILE") {
        0 => (),
        1 => {
            let safe_file: String = make_safe(read_bytes(&matches.value_of("IMPORT_FILE").unwrap()));
            let mut path: &Path = Path::new(matches.value_of("IMPORT_FILE").unwrap());
            db.samples.push(Sample{filename: String::from(path.file_stem().unwrap().to_str().unwrap()), data: safe_file, md5: String::new(), sha256: String::new()});
            ()
        },
        _ => (),
    }

    match matches.occurrences_of("EXPORT_FILE") {
        0 => (),
        1 => {
            let mut live_bytes: Vec<u8> = vec!();
            for sample in &db.samples {
                if &sample.filename == matches.value_of("EXPORT_FILE").unwrap() {
                    live_bytes = make_live(&sample.data);
                } else {
                    println!("Sample not found.")
                }
            }
            File::create(matches.value_of("EXPORT_FILE").unwrap()).expect("Failed to create file.").write(live_bytes.as_slice());
            ()
        },
        _ => (),
    }
    save_db(db);
}
