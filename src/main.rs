#[macro_use] extern crate log;
extern crate simplelog;
extern crate tsplib;
extern crate rand;

mod utils;
mod solver;
use solver::colony::Colony;


fn set_logger(level: u64) {
    use simplelog::*;

    let log_level: LevelFilter = match level + 1 {
        0 => LevelFilter::Off,
        1 => LevelFilter::Error,
        2 => LevelFilter::Warn,
        3 => LevelFilter::Info,
        4 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };

    TermLogger::init(log_level, Config::default(), TerminalMode::Stderr).unwrap();
}

fn main() {
    const NAME: &'static str = env!("CARGO_PKG_NAME");
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
    const ABOUT: &'static str = env!("CARGO_PKG_DESCRIPTION");

    let matches = clap::App::new(NAME)
        .version(VERSION)
        .author(AUTHORS)
        .about(ABOUT)
        .arg(
            clap::Arg::with_name("verbose")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity")
        )
        .arg(
            clap::Arg::with_name("input")
                .help("The input file")
                .required(true)
                .index(1)
        )
        .arg(
            clap::Arg::with_name("n_ants")
                .help("The number of ants")
                .required(true)
                .index(2)
        )
        .arg(
            clap::Arg::with_name("n_inter")
                .help("The number of interations")
                .required(true)
                .index(3)
        )
        .get_matches();

    set_logger(matches.occurrences_of("verbose"));
    info!("Everything done!");

    let file_path = matches.value_of("input").unwrap();
    let instance = utils::loader::open(file_path).unwrap();
    info!("Instance \"{}\" loaded", instance.name);

    let n_ants = matches.value_of("n_ants").unwrap().parse::<usize>().unwrap();
    let n_inter = matches.value_of("n_inter").unwrap().parse::<usize>().unwrap();

    let mut colony = Colony::new(instance, n_ants, n_inter, 1f64, 1f64, 0.5);
    let result = colony.run(0);

    println!("{:#?}", result);
}
