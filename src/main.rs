use egg_smol::EGraph;
use std::io::Read;

fn main() {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info)
        .format_timestamp(None)
        .format_target(false)
        .parse_default_env()
        .init();

    let mut egraph = EGraph::default();
    // egraph.
    let mut program = String::new();
    std::fs::File::open("egglog_src/main.egg")
        .unwrap()
        .read_to_string(&mut program)
        .unwrap();
    match egraph.parse_and_run_program(&program) {
        Ok(_msgs) => {}
        Err(err) => {
            eprintln!("{}", err);
        }
    }

    std::process::exit(1)
}
