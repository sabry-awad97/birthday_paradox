use plotters::prelude::{ChartBuilder, IntoDrawingArea, SVGBackend};
use plotters::series::LineSeries;
use plotters::style::{IntoFont, RED};
use rand::Rng;
use rayon::prelude::*;
use std::collections::HashSet;

fn generate_random_birthday(leap_year: bool) -> String {
    let mut rng = rand::thread_rng();
    let month = rng.gen_range(1..=12);
    let day = match month {
        2 => {
            if leap_year {
                rng.gen_range(1..=29)
            } else {
                rng.gen_range(1..=28)
            }
        }
        4 | 6 | 9 | 11 => rng.gen_range(1..=30),
        _ => rng.gen_range(1..=31),
    };

    format!("{:02}/{:02}", month, day)
}

fn generate_random_group(size: usize) -> Vec<String> {
    // Generate a group of random birthdays
    let mut rng = rand::thread_rng();
    let year = rng.gen_range(1900..=2100);
    let is_leap_year = |year: i32| year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    let leap_year = is_leap_year(year);
    (0..size)
        .into_par_iter()
        .map(|_| generate_random_birthday(leap_year))
        .collect()
}

fn check_for_duplicates(birthdays: &[String]) -> bool {
    // Check if there are any duplicate birthdays in the group
    let mut unique_birthdays = HashSet::new();
    birthdays
        .iter()
        .any(|birthday| !unique_birthdays.insert(birthday.clone()))
}

fn main() {
    println!("Welcome to the Birthday Paradox Checker!");

    let num_trials = get_num_trials_from_user();

    let group_sizes = [23, 30, 50, 70];

    let prob_data = group_sizes
        .into_iter()
        .map(|group_size| {
            let total_matches = (0..num_trials)
                .into_par_iter()
                .filter(|_| {
                    let birthdays = generate_random_group(group_size);
                    check_for_duplicates(&birthdays)
                })
                .count();

            let probability = (total_matches as f64 / num_trials as f64) * 100.0;

            println!(
                "For a group size of {}, Probability: {:.2}%",
                group_size, probability
            );

            (group_size, probability)
        })
        .collect::<Vec<_>>();

    render_plot(&prob_data);
}

fn render_plot(data: &[(usize, f64)]) {
    let root = SVGBackend::new("probability_vs_group_size.svg", (800, 600)).into_drawing_area();
    let mut chart = ChartBuilder::on(&root)
        .caption("Probability vs Group Size", ("sans-serif", 30))
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(2..100, 0.0..100.0)
        .unwrap();

    chart
        .configure_mesh()
        .x_desc("Group Size")
        .y_desc("Probability (%)")
        .axis_desc_style(("sans-serif", 15).into_font())
        .draw()
        .unwrap();

    chart
        .draw_series(LineSeries::new(
            data.iter().map(|&(x, y)| (x as i32, y)),
            &RED,
        ))
        .unwrap();
}

fn get_num_trials_from_user() -> usize {
    // Prompt the user for the number of trials and validate the input
    loop {
        println!("Enter the number of trials: ");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        match input.trim().parse() {
            Ok(trials) if trials > 0 => return trials,
            _ => {
                println!("Please enter a valid number of trials (greater than 0).");
                continue;
            }
        };
    }
}
