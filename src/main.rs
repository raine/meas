mod opts;
mod time;

use std::{
    io::{self, BufRead, BufReader, Write},
    time::Instant,
};

use duct::cmd;
use hhmmss::Hhmmss;
use humantime::format_duration;

fn print_line(start_instant: &Instant, line: &str) {
    let elapsed = start_instant.elapsed();
    let timer = elapsed.hhmmssxxx();
    let timer_without_hours = &timer[3..];

    print!("{} │ {}\r\n", timer_without_hours, line);
    io::stdout().flush().unwrap();
}

fn make_duct_exp(config: &opts::Opt) -> duct::Expression {
    let command = &config.command[0];
    let command_args = config.command[1..].to_vec();
    cmd(command, command_args).stderr_to_stdout()
}

fn main() {
    let config = opts::parse_args();
    if config.command.is_empty() {
        opts::print_help();
        std::process::exit(1);
    }

    let duct_exp = make_duct_exp(&config);
    let start_instant = Instant::now();
    let reader = match duct_exp.reader() {
        Ok(child) => child,
        Err(err) => {
            eprintln!("[error] failed to start program:\n{}", err);
            std::process::exit(1);
        }
    };

    let lines = BufReader::new(reader).lines();
    for line_result in lines {
        match line_result {
            Ok(line) => print_line(&start_instant, &line),
            Err(_) => break,
        }
    }

    println!(
        "command completed in {}",
        format_duration(time::duration_without_ns(&start_instant.elapsed()))
    );
}
