use std::{io, process};

mod reader;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;

#[derive(Debug)]
enum Part {
  One,
  Two,
  All,
}

fn main() {
  let mut i = String::new();
  println!("Day:");
  match io::stdin().read_line(&mut i) {
    Ok(_) => {
      let input = i.as_str().trim_end();
      handle_input(input);
    }
    Err(error) => println!("error: {}", error),
  }
}

fn handle_input(input: &str) {
  match input {
    "exit" => process::exit(0),
    "all" => {
      day01::all();
      day02::all();
      day03::all();
    }
    "1" => match handle_part() {
      Ok(Part::One) => day01::part1::run(),
      Ok(Part::Two) => day01::part2::run(),
      Ok(Part::All) => day01::all(),
      Err(err) => eprintln!("{:?}", err),
    },
    "2" => match handle_part() {
      Ok(Part::One) => day02::part1::run(),
      Ok(Part::Two) => day02::part2::run(),
      Ok(Part::All) => day02::all(),
      Err(err) => eprintln!("{:?}", err),
    },
    "3" => match handle_part() {
      Ok(Part::One) => day03::part1::run(),
      Ok(Part::Two) => day03::part2::run(),
      Ok(Part::All) => day03::all(),
      Err(err) => eprintln!("{:?}", err),
    },
    "4" => match handle_part() {
      Ok(Part::One) => day04::part1::run(),
      Ok(Part::Two) => day04::part2::run(),
      Ok(Part::All) => day04::all(),
      Err(err) => eprintln!("{:?}", err),
    },
    "5" => match handle_part() {
      Ok(Part::One) => day05::part1::run(),
      Ok(Part::Two) => day05::part2::run(),
      Ok(Part::All) => day05::all(),
      Err(err) => eprintln!("{:?}", err),
    },
    "6" => match handle_part() {
      Ok(Part::One) => day06::part1::run(),
      Ok(Part::Two) => day06::part2::run(),
      Ok(Part::All) => day06::all(),
      Err(err) => eprintln!("{:?}", err),
    },
    "7" => match handle_part() {
      Ok(Part::One) => day07::part1::run(),
      Ok(Part::Two) => day07::part2::run(),
      Ok(Part::All) => day07::all(),
      Err(err) => eprintln!("{:?}", err),
    },
    "8" => match handle_part() {
      Ok(Part::One) => day08::part1::run(),
      Ok(Part::Two) => day08::part2::run(),
      Ok(Part::All) => day08::all(),
      Err(err) => eprintln!("{:?}", err),
    },
    "9" => match handle_part() {
      Ok(Part::One) => day09::part1::run(),
      Ok(Part::Two) => day09::part2::run(),
      Ok(Part::All) => day09::all(),
      Err(err) => eprintln!("{:?}", err),
    },
    _ => eprintln!("day is invalid"),
  }
}

fn handle_part() -> Result<Part, &'static str> {
  let mut i = String::new();
  println!("Part:");
  match io::stdin().read_line(&mut i) {
    Ok(_) => match i.as_str().trim_end() {
      "1" => Ok(Part::One),
      "2" => Ok(Part::Two),
      "all" => Ok(Part::All),
      _ => Err("Part is invalid"),
    },
    Err(_) => Err("error"),
  }
}
