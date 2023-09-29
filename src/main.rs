use std::env;
use std::error::Error;
use std::process;

fn main() {
    if let Err(err) = try_main() {
        eprintln!("{}", err);
        process::exit(2);
    }
}

fn try_main() -> Result<(), Box<dyn Error>>{
    const HELP: &str = "\
        Alternative for Cargo\n\n\
        Usage: freight [COMMAND] [OPTIONS]\n\n\
        Commands:\n    \
            build    Build a Freight or Cargo project
            test     Test a Freight or Cargo project
            help     Print out this message
    ";
    let mut args = env::args().skip(1);
    match args.next().as_ref().map(String::as_str) {
        Some("build") => freight::build()?,
        Some("test") => {
            freight::build_tests()?;
            freight::run_tests()?;
        }
        Some("help") => println!("{HELP}"),
        _ => {
            println!("unsupported command");
            println!("{HELP}");
            process::exit(1);
        }
    }
    Ok(())
}
