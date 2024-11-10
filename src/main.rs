use clap::Parser;
use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use std::{env, error::Error, fmt::Debug, fs::File, io, path::{self, PathBuf}, process::{Command, Output}, string, time::{SystemTime, UNIX_EPOCH}};
use chrono::{DateTime, Datelike, NaiveDateTime};

const DEFAULT_PET_DIR: &str = "~/.config/first.pet";


#[derive(Debug)]
enum FileError {
    ErrorOpening,
    DumpError,
    AlreadyExists
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {


    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,

    #[arg(short, long)]
    info: bool,

    #[arg(long)]
    init: bool,

    #[arg(long)]
    path: Option<String>,

}

struct Pet {
    name: String,
    birth: i64,
    hunger: u8,
    happiness: u8,
    last_save: i64,
    path: String, //its a full path
}

fn main() {
    let args = Args::parse();

    let mut DB_PATH= std::env::var("TPET_FILE_PATH");
    if DB_PATH.is_err() {
        DB_PATH = Ok(DEFAULT_PET_DIR.to_string());
    }
    if args.path.is_some() {
        DB_PATH = Ok(args.path.unwrap());
    }
    let DB_PATH = expand_home(DB_PATH.unwrap());

    if args.init {
        match create_pet(DB_PATH) {
            Ok(pet) => {
                println!("Pet created at {} in {}, it's name is {:?}", pet.birth,pet.path, pet.name);
            },
            Err(e) => {
                println!("Error creating pet: {:?}", e);
            }
        };
        return;
    }

    let db = open_db(DB_PATH);
    if args.info {
        let name: String = db.get::<String>("name").unwrap();
        let birth = db.get::<i64>("birth").unwrap();
        let hunger = db.get::<u8>("hunger").unwrap();
        let happiness = db.get::<u8>("happiness").unwrap();
        let last_save = db.get::<i64>("last_save").unwrap();

        let difference_in_seconds = SystemTime::now().duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs() as i64 - last_save ;

        println!("Pet {} was born at {}, it's hunger is at {} and it's happiness is at {}. Last save was {} ago", name, timestamp_to_dmy(birth), hunger, happiness, seconds_to_dhms(difference_in_seconds));
    }

}

fn create_pet(path: String) -> Result<Pet,FileError> {

  



    let stdin = io::stdin();
    let mut name = String::new();
    let hunger = 100;
    let happiness = 100;
    let current_unix_timestamp:u64 = SystemTime::now().duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs().into();


    println!("What is your pet's name?");
    stdin.read_line(&mut name).unwrap();
    let name = name.trim();

    let pet = Pet {
        name: name.to_string(),
        birth: current_unix_timestamp as i64,
        hunger,
        happiness,
        last_save: current_unix_timestamp as i64,
        path: path.clone()
    };

    // Check if file exists
    let path = path::Path::new(&path);
    if path.exists() {
        return Err(FileError::AlreadyExists);
    }

    let mut db = PickleDb::new(&path, PickleDbDumpPolicy::DumpUponRequest, SerializationMethod::Json);
    db.set("name", &pet.name);
    db.set("birth", &pet.birth);
    db.set("hunger", &pet.hunger);
    db.set("happiness", &pet.happiness);
    db.set("last_save", &pet.last_save);
    
    //db.dump() or return dumperror in the function
    match db.dump() {
        Ok(()) => return Ok(pet),
        Err(_) => return Err(FileError::DumpError)
    };


}

fn open_db(path: String) -> PickleDb {
    let DB_PATH = path::Path::new(&path);

    println!("DB_PATH: {}", DB_PATH.display());

    PickleDb::load(DB_PATH, PickleDbDumpPolicy::AutoDump, SerializationMethod::Json).expect(&format!("Error loadind the petfile at {}",DB_PATH.display().to_string()))
}

fn expand_home(mut path: String) -> String {
    if path.starts_with("~") {
        let home_dir = if cfg!(target_os = "windows") {
            env::var("USERPROFILE").ok().unwrap()
        } else {
            env::var("HOME").ok().unwrap()
        };
        path = path.replace("~", &home_dir);
    }
    path
}
fn seconds_to_dhms(mut seconds: i64) -> String {
    let days = seconds / 86400;
    seconds %= 86400;
    let hours = seconds / 3600;
    seconds %= 3600;
    let minutes = seconds / 60;
    seconds %= 60;
    format!("{}d {}h {}m {}s", days, hours, minutes, seconds)
}


fn timestamp_to_dmy(timestamp: i64) -> String {
    let datetime = DateTime::from_timestamp(timestamp, 0).unwrap();
    
    let day = datetime.day();
    let month = datetime.month();
    let year = datetime.year();
    
    format!("{:02}-{:02}-{}", day, month, year)
}
