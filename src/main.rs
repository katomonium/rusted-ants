#[macro_use] extern crate log;
extern crate simplelog;
extern crate tsplib;
extern crate rand;

mod utils;
mod ant;
use utils::sparse_matrix::SparseMatrix;
use ant::Ant;

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
                .help("Sets the level of verbosity"),
        )
        .arg(
            clap::Arg::with_name("input")
                .help("The input file")
                .required(true)
                .index(1),
        )
        .get_matches();

    set_logger(matches.occurrences_of("verbose"));

    let file_path = matches.value_of("input").unwrap();
    let r = utils::loader::open(file_path);

    let matrix = SparseMatrix::new_from_instace(r.unwrap());

    let mut pheromone = SparseMatrix::new(matrix.size(), matrix.size());
    for i in 0..matrix.size() {
        for j in 0..(i+1) {
            pheromone.set(i, j, 1f64);
        }
    }

    let mut ant = Ant::new(1f64, 3f64, &matrix, &pheromone);

    ant.find_circuit(0);

    println!("{:?}", ant.path);
}
