use rand::Rng;
use std::collections::HashSet;
use std::time::Instant;

fn generate_random_birthday() -> String {
    // Generate a random birthday in MM/DD format
    let month = rand::thread_rng().gen_range(1..=12);
    let day = rand::thread_rng().gen_range(1..=31);
    format!("{:02}/{:02}", month, day)
}

fn generate_random_group(size: usize) -> Vec<String> {
    // Generate a group of random birthdays
    (0..size).map(|_| generate_random_birthday()).collect()
}

fn check_for_duplicates(birthdays: &[String]) -> bool {
    // Check if there are any duplicate birthdays in the group
    let mut unique_birthdays = HashSet::new();
    for birthday in birthdays.iter() {
        if !unique_birthdays.insert(birthday.clone()) {
            return true;
        }
    }
    false
}

fn main() {
    println!("Welcome to the Birthday Paradox Checker!");

    // Ask the user for input
    let group_size = get_group_size_from_user();
    let num_trials = get_num_trials_from_user();

    let start_time = Instant::now();
    let mut total_matches = 0;

    // Run multiple trials
    for i in 0..num_trials {
        if i != 0 && i % 10000 == 0 {
            println!("{} trials run...", i);
        }
        let birthdays = generate_random_group(group_size);
        if check_for_duplicates(&birthdays) {
            total_matches += 1;
        }
    }

    let end_time = Instant::now(); // Stop measuring execution time
    let elapsed_time = end_time.duration_since(start_time);

    let probability = (total_matches as f64 / num_trials as f64) * 100.0;

    // Display results
    println!("{} trials run...", num_trials);
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
