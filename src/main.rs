use fingerprint_guard::{db::Store, enroll_file, check_file, CheckResult};
use std::{env, error::Error, path::Path};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 || !matches!(args[1].as_str(), "enroll" | "check") {
        eprintln!("Usage: {} <enroll|check> <database.db> <file>", args[0]);
        std::process::exit(2);
    }
    let store = Store::open(Path::new(&args[2]))?;
    let file = Path::new(&args[3]);
    match args[1].as_str() {
        "enroll" => println!("Enrolled SHA-256: {}", enroll_file(&store, file)?),
        "check" => match check_file(&store, file)? {
            CheckResult::Match => println!("MATCH: file unchanged"),
            CheckResult::Modified => { println!("MODIFIED: file content differs"); std::process::exit(1); },
            CheckResult::NotEnrolled => { println!("NOT ENROLLED: no baseline for file"); std::process::exit(3); },
        },
        _ => unreachable!(),
    }
    Ok(())
}
