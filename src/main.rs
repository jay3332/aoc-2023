use dotenv_codegen::dotenv;
use std::{
    env, fs,
    path::Path,
    process::{Command, Stdio},
};

#[inline]
fn parser() -> getopts::Options {
    let mut parser = getopts::Options::new();
    parser.optopt("d", "day", "Which day to run (1-25)", "DAY");
    parser.optflagopt("n", "new", "Create a new crate for the day", "DAY");
    parser.optflag("h", "help", "Print this help menu");
    parser.optflag("D", "debug", "Run in debug mode");
    parser.optflag("2", "part-two", "Run part two");
    parser
}

fn create_day(day: u8) -> Result<(), Box<dyn std::error::Error>> {
    let dir = format!("day_{day:02}");
    let path = Path::new(&dir);

    if path.is_dir() {
        println!("Day {day} already exists");
        return Ok(());
    }

    let cargo_toml = fs::read_to_string("Cargo.toml")?
        .lines()
        .map(ToString::to_string)
        .take_while(|line| !line.starts_with("[workspace]"))
        .chain(Some("[workspace]\nmembers = [\n    \"runner\",".to_string()))
        .chain((1..=day).map(|day| format!(r#"    "day_{day:02}","#)))
        .chain(Some("]".to_string()))
        .collect::<Vec<_>>()
        .join("\n");

    fs::write("Cargo.toml", cargo_toml)?;
    fs::create_dir_all(path.join("src"))?;
    fs::write(
        path.join("Cargo.toml"),
        format!(
            r#"[package]
name = "day_{day:02}"
version = "0.1.0"
edition = "2021"
authors = ["jay3332"]

[dependencies]"#,
        ),
    )?;
    fs::write(
        path.join("src").join("lib.rs"),
        r#"#![feature(never_type)]
#![allow(unused_variables)]

/// Make sure outputs impl Display!
type Output<T> = Result<T, Box<dyn std::error::Error>>;

pub fn part_1(input: &'static str) -> Output<!> {
    todo!()
}

pub fn part_2(input: &'static str) -> Output<!> {
    todo!()
}
"#,
    )?;

    if !Path::new(format!("input/day_{day:02}.txt").as_str()).exists() {
        fs::create_dir_all("input")?;

        Command::new("curl")
            .arg(format!("https://adventofcode.com/2023/day/{day}/input"))
            .args(["-b", format!("session={}", dotenv!("AOC_SESSION")).as_str()])
            .stdout(fs::File::create(format!("input/day_{day:02}.txt"))?)
            .output()?;
    }

    println!("Created day {day}");
    Ok(())
}

/// Runner interface, ignore this
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parser = parser();
    let matches = parser.parse(env::args().skip(1))?;

    if matches.opt_present("h") {
        let brief = format!(
            "Usage: {} [options]",
            env::current_exe().unwrap().to_string_lossy()
        );
        print!("{}", parser.usage(&brief));
        return Ok(());
    }

    if let Some(day) = matches.opt_str("n") {
        create_day(day.parse()?)?;
        return Ok(());
    }

    let day = if let Some(day) = matches.opt_str("d") {
        day.parse::<u8>()?
    } else {
        println!("No day specified, aborting");
        return Ok(());
    };

    fs::create_dir_all("runner/src")?;
    fs::write(
        "runner/Cargo.toml",
        format!(
            r#"[package]
name = "runner"
version = "0.1.0"
edition = "2021"
authors = ["jay3332"]

[dependencies]
day_{0:02} = {{ path = "../day_{0:02}" }}"#,
            day,
        ),
    )?;
    fs::write(
        "runner/src/main.rs",
        format!(
            r#"fn main() {{
    println!("=== AoC Day {0:02} Part {1} ===\n");
    let instant = std::time::Instant::now();
    let out = day_{0:02}::part_{1}(include_str!("../../input/day_{0:02}.txt"));

    match out {{
        Ok(out) => println!("Output\n======\n{{}}\n", out),
        Err(err) => println!("Error\n=====\n{{:?}}\n", err),
    }}
    println!("Execution time: {{:?}}", instant.elapsed());
}}
"#,
            day,
            if matches.opt_present("2") { "2" } else { "1" },
        ),
    )?;

    let mut command = Command::new("cargo");
    command
        .arg("build")
        .args(["-p", "runner"])
        .stderr(Stdio::inherit());

    let path = if matches.opt_present("D") {
        "target/debug/runner"
    } else {
        command.arg("--release");
        "target/release/runner"
    };
    command.output()?;

    if !Path::new(path).exists() {
        eprintln!("runner failed to compile!");
        return Ok(());
    }

    Command::new(path)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;

    fs::remove_file(path)?;
    Ok(())
}
