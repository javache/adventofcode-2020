use std::collections::HashMap;
use std::io::{self, BufRead};

fn read_numbers() -> io::Result<Vec<u64>> {
    let mut result = Vec::new();
    for line in io::stdin().lock().lines() {
        if let Ok(number) = line?.parse::<u64>() {
            result.push(number);
        }
    }
    Ok(result)
}

fn main() -> io::Result<()> {
    let mut input = read_numbers()?;
    input.sort();

    let mut differences = [0, 0, 1];
    for (jolts, prev_jolts) in input.iter().zip([0].iter().chain(input.iter())) {
        let delta = jolts - prev_jolts - 1;
        differences[delta as usize] += 1;
    }
    println!(
        "(1) {:?} => {}",
        differences,
        differences[0] * differences[2]
    );

    fn count_arrangements(
        current_jolts: u64,
        next_options: &[u64],
        lookup: &mut HashMap<u64, usize>,
    ) -> usize {
        if next_options.len() == 0 {
            1
        } else if let Some(count) = lookup.get(&current_jolts) {
            *count
        } else {
            let count = next_options
                .iter()
                .enumerate()
                .take_while(|(_, &option)| option - current_jolts <= 3)
                .map(|(idx, &option)| {
                    count_arrangements(option, &next_options[(idx + 1)..], lookup)
                })
                .sum();
            lookup.insert(current_jolts, count);
            count
        }
    }

    let mut arrangements_seen = HashMap::new();
    println!(
        "(2) There are {} arrangements",
        count_arrangements(0, &input, &mut arrangements_seen)
    );

    Ok(())
}
