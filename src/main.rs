use clap::Parser;
use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use rand::Rng;
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


    /// Show information about your pet.
    #[arg(short, long)]
    info: bool,

    /// Initialize a new pet.
    #[arg(long)]
    init: bool,

    /// Path to the petfile if using a custom one, defaults to ~/.config/first.pet TPET_FILE_PATH env. 
    #[arg(long)]
    path: Option<String>,

    /// Play a little with your pet!.
    #[arg(short, long)]
    play: bool,

    /// Feed your pet!.
    #[arg(short, long)]
    feed: bool,

    /// Check on your pet.
    #[arg(short, long)]
    greet: bool

}

struct Pet {
    name: String,
    birth: i64,
    hunger: u8,
    happiness: u8,
    last_save: i64,
    path: String, //its a full path
    db: PickleDb
}

impl Pet {
    fn init(&mut self) -> Result<(), FileError> {
        
        let mut db = &mut self.db;
        db.set("name", &self.name);
        db.set("birth", &self.birth);
        db.set("hunger", &self.hunger);
        db.set("happiness", &self.happiness);
        db.set("last_save", &self.last_save);
        db.set("path", &self.path);
        match db.dump() {
            Ok(()) => return Ok(()),
            Err(_) => return Err(FileError::DumpError)
        }
    }
    fn feed(&mut self, amount: i8) -> Result<(), FileError> {

        let mut updated_hunger: i8 = self.hunger as i8 + amount;
        if updated_hunger < 0 {
            updated_hunger = 0;
        } else if updated_hunger > 100 {
            updated_hunger = 100;
        }
        let updated_hunger = updated_hunger as u8;

        let db = &mut self.db;
        db.set("hunger", &updated_hunger);
        db.set("last_save", &unix_now());
        match db.dump() {
            Ok(()) => return Ok(()),
            Err(_) => return Err(FileError::DumpError)
        }
    }
    fn play(&mut self, amount: i8) -> Result<(), FileError> {

        let mut updated_happiness: i8 = self.happiness as i8 + amount;
        if updated_happiness < 0 {
            updated_happiness = 0;
        } else if updated_happiness > 100 {
            updated_happiness = 100;
        }
        let updated_happiness = updated_happiness as u8;

        let db = &mut self.db;
        db.set("happiness", &updated_happiness);
        db.set("last_save", &unix_now());
        match db.dump() {
            Ok(()) => return Ok(()),
            Err(_) => return Err(FileError::DumpError)
        }
    }
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
    let mut pet = load_pet(db);
    if args.info {

        let difference_in_seconds = unix_now()  - &pet.last_save ;

        println!("Pet {} was born at {}, it's hunger is at {} and it's happiness is at {}. Last save was {} ago", &pet.name, timestamp_to_dmy(pet.birth), &pet.hunger, &pet.happiness, seconds_to_dhms(difference_in_seconds));
    }

    let interactable =  pet.last_save - unix_now() < 60;

    if args.play {

        if !interactable {
            println!("{} had too much for for now! Give them a break", &pet.name);
            return;
        }

        match pet.play(10) {
            Ok(()) => {
                println!("{} feels joy now! happiness increased by 10", &pet.name);
            },
            Err(e) => {
                println!("Error playing with pet: {:?}", e);
            }
        }
    }
    if args.feed {
        match pet.feed(-10) {
            Ok(()) => {
                println!("{} is full right now! hunger decreased by 10", &pet.name);
            },
            Err(e) => {
                println!("Error feeding pet: {:?}", e);
            }
        }
    }
    if args.greet {
        greet_pet(pet);
    }

}

fn greet_pet(pet: Pet) {
    let hungry_phrases = vec!["Felling really hungry rn!", "I want food!", "I'm starving! (", "I'm famished!"];
    let bored_phrases = vec!["I'm bored!", "I want to play!", "I'm tired of doing nothing!", "I'm feeling lonely!"];
    let average_phrases = vec!["I'm feeling good!", "How you doing?", "I'm fine, thanks for asking!", "Remember to check on me often!"];
    let advices = vec!["Remember to stage your files before commiting!", "Remember to push your changes!", "Remember to pull before pushing!", "Remember to drink some water!"];

    if pet.hunger > 50 {
        println!("{}", hungry_phrases[rand::thread_rng().gen_range(0..hungry_phrases.len())]);
    } else if pet.happiness < 50 {
        println!("{}", bored_phrases[rand::thread_rng().gen_range(0..bored_phrases.len())]);
    } else {
        if rand::thread_rng().gen_bool(0.5) {
            println!("{}", average_phrases[rand::thread_rng().gen_range(0.. average_phrases.len())]);
        } else {
            println!("{}", advices[rand::thread_rng().gen_range(0..advices.len())]);
        }
    } 
}

fn load_pet(db: PickleDb) -> Pet {
    let name: String = db.get::<String>("name").unwrap();
    let birth = db.get::<i64>("birth").unwrap();
    let hunger = db.get::<u8>("hunger").unwrap();
    let happiness = db.get::<u8>("happiness").unwrap();
    let last_save = db.get::<i64>("last_save").unwrap();
    //let path = db.get::<String>("path").unwrap();
    let path = String::from("value");

    Pet {
        name,
        birth,
        hunger,
        happiness,
        last_save,
        path,
        db
    }
}

fn create_pet(path: String) -> Result<Pet,FileError> {

    let mut name = String::new();
    let hunger = 0;
    let happiness = 100; 
    let current_unix_timestamp:i64 = unix_now();


    println!("What is your pet's name? \n > ");
    let stdin = io::stdin();

    stdin.read_line(&mut name).unwrap();
    let name = name.trim();


    // Check if file exists
    let path = path::Path::new(&path);
    if path.exists() {
        return Err(FileError::AlreadyExists);
    }

    let db = PickleDb::new(&path, PickleDbDumpPolicy::DumpUponRequest, SerializationMethod::Json);
    let mut pet = Pet {
        name: name.to_string(),
        birth: current_unix_timestamp as i64,
        hunger,
        happiness,
        last_save: current_unix_timestamp as i64,
        path: path.to_str().unwrap().to_string(),
        db
    };

    
    //db.dump() or return dumperror in the function
    match pet.init() {
        Ok(()) => return Ok(pet),
        Err(_) => return Err(FileError::DumpError)
    };


}

fn open_db(path: String) -> PickleDb {
    let DB_PATH = path::Path::new(&path);
    if !DB_PATH.exists() {
        panic!("Petfile not found at {}", DB_PATH.display().to_string());
    }
    PickleDb::load(DB_PATH, PickleDbDumpPolicy::DumpUponRequest, SerializationMethod::Json).expect(&format!("Error loadind the petfile at {}",DB_PATH.display().to_string()))
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

#[inline(always)] fn unix_now() -> i64 { SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_secs() as i64 }