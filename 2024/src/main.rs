mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;

mod utilities;

use clap::Parser;

#[derive(Parser)]
struct Cli {
    // Day to run
    #[arg(short = 'd', long)]
    day: Option<usize>,

    // Part to run
    #[arg(short = 'p', long)]
    part: Option<usize>,
}

fn main() -> Result<(), String> {
    let args = Cli::parse();

    let puzzles = [
        [day_01::part_1, day_01::part_2],
        [day_02::part_1, day_02::part_2],
        [day_03::part_1, day_03::part_2],
        [day_04::part_1, day_04::part_2],
        [day_05::part_1, day_05::part_2],
        [day_06::part_1, day_06::part_2],
        [day_07::part_1, day_07::part_2],
        [day_08::part_1, day_08::part_2],
        [day_09::part_1, day_09::part_2],
        [day_10::part_1, day_10::part_2],
        [day_11::part_1, day_11::part_2],
        [day_12::part_1, day_12::part_2],
        [day_13::part_1, day_13::part_2],
        [day_14::part_1, day_14::part_2],
    ];

    let days_to_run = match args.day {
        Some(day) if day <= puzzles.len() => day..=day,
        Some(day) => return Err(format!("day {} not yet registered in main.rs", day)),
        None => 1..=puzzles.len(),
    };

    let parts_to_run = match args.part {
        Some(part) if part == 1 || part == 2 => part..=part,
        Some(part) => return Err(format!("part {} does not exist", part)),
        None => 1..=2,
    };

    for day in days_to_run {
        let contents = utilities::read_input(day)?;
        for part in parts_to_run.clone() {
            let func = puzzles[day - 1][part - 1];
            println!("{}", func(&contents)?);
        }
    }

    Ok(())
}
