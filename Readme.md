# Birthday Paradox Simulator

A Rust program for experimentally simulating the famous "Birthday Paradox" problem through random testing.

## Overview

The Birthday Paradox, also known simply as the Birthday Problem, demonstrates the surprising probability of duplicates when randomly selecting people's birthdays. Intuitively, one might expect the chance of a match to be low with small groups, but it increases rapidly.

This program validates the paradox through randomized Monte Carlo simulation. It generates random birthdays, checks groups for duplicates, and analyzes results over many trials to determine the actual probability of matches for different sized random samples.

## Usage

### Building

To build the executable, run:

```bash
cargo build --release
```

### Running

Execute the binary and provide inputs when prompted:

```bash
./target/release/birthday_paradox
```

Inputs:

- Group size to simulate
- Number of trials to perform

### Output

The program outputs:

- Total matches found
- Calculated probability percentage
- Execution time

## Algorithm

Pseudocode overview:

1. Generate random birthdays in MM/DD format
2. Create a group by repeating above
3. Check group for duplicate birthdays
4. Track number of matches
5. Repeat process for number of trials
6. Calculate final probability

It utilizes a HashSet for efficient duplicate checking.

## Sample Results

A sampling of results from 100,000 trials:

### Group Size: 23

Matches: 50212
Probability: 50.21%

### Group Size: 30

Matches: 69847
Probability: 69.85%

### Group Size: 50

Matches: 96868
Probability: 96.87%

## Contributors

Written by Sabry Awad to demonstrate Monte Carlo simulation techniques in Rust.
