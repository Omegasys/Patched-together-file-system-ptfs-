mod mkfs;
mod mount;
mod snapshot;
mod scrub;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        return;
    }

    match args[1].as_str() {
        "mkfs" => mkfs::run(),
        "mount" => mount::run(),
        "snapshot" => snapshot::run(),
        "scrub" => scrub::run(),
        _ => print_help(),
    }
}

fn print_help() {
    println!("PTFS CLI");
    println!("Commands:");
    println!("  mkfs        Create filesystem");
    println!("  mount       Mount filesystem");
    println!("  snapshot    Create snapshot");
    println!("  scrub       Run integrity scrub");
}
