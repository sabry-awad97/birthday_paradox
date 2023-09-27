use rand::Rng;
use rayon::prelude::*;
use std::collections::HashSet;
use std::time::Instant;

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

    // Ask the user for input
    let group_size = get_group_size_from_user();
    let num_trials = get_num_trials_from_user();

    let start_time = Instant::now();

    // Run multiple trials
    let total_matches = (0..num_trials)
        .into_par_iter()
        .filter(|_| {
            let birthdays = generate_random_group(group_size);
            check_for_duplicates(&birthdays)
        })
        .count();

    let end_time = Instant::now(); // Stop measuring execution time
    let elapsed_time = end_time.duration_since(start_time);

    let probability = (total_matches as f64 / num_trials as f64) * 100.0;

    // Display results
    println!(
        "\nResults after {} trials with a group size of {}:",
        num_trials, group_size
    );
    println!("Total matches found: {}", total_matches);
    println!(
        "Probability of at least two people sharing a birthday: {:.2}%",
        probability
    );
    println!("Elapsed time: {:?}", elapsed_time);
}

fn get_group_size_from_user() -> usize {
    // Prompt the user for the group size and validate the input
    loop {
        println!("Enter the size of the group (at least 2): ");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        match input.trim().parse() {
            Ok(size) if size >= 2 => return size,
            _ => {
                println!("Please enter a valid group size (at least 2).");
                continue;
            }
        };
    }
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
