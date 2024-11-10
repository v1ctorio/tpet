use clap::Parser;
use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use std::{error::Error, process::{Command, Output}};
/// Simple program to greet a pers

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {


    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,

    #[arg(short, long)]
    status: bool

}

struct Pet {
    name: String,
    age: u8,
    hunger: u8,
    happiness: u8,
}

fn main() {


    let args = Args::parse();
    if args.status {
        let output = Command::new("cowsay")
            .output()
            .expect("Cowsay is not installed!");
        println!("status: {}", output.status);
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    }

}

fn open_db() -> PickleDb {
    let mut DB_PATH= std::env::var("TPET_FILE_PATH");
    if DB_PATH.is_err() {
        DB_PATH = Ok("pet.db".to_string());
    }
    let DB_PATH = DB_PATH.unwrap();

    println!("DB_PATH: {}", DB_PATH);

    let db = PickleDb::load("pet.db", PickleDbDumpPolicy::AutoDump, SerializationMethod::Json).unwrap();
    db
}