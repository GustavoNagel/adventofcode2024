use std::collections::HashMap;

use counter::Counter;

fn mix(secret_number: i64, value: i64) -> i64 {
    secret_number ^ value
}

fn prune(secret_number: i64) -> i64 {
    secret_number % 16777216
}

fn calculate_secret_key(old_secret_key: i64) -> i64 {
    let mut secret_key: i64 = old_secret_key;
    // Step 1
    secret_key = prune(mix(secret_key, secret_key * 64));
    // Step 2
    let temp_result = (secret_key as f64 / 32.0).floor() as i64;
    secret_key = prune(mix(secret_key, temp_result));
    // Step 3
    secret_key = prune(mix(secret_key, secret_key * 2048));
    secret_key
}

pub fn run(contents: String, part: &i8) {
    if part == &1 {
        let total_sum: i64 = contents.lines().map(|line| {
            let mut secret_key: i64 = line.parse().unwrap();
            for _ in 0..2000 {
                secret_key = calculate_secret_key(secret_key);
            }
            // println!("Line {} Secret key: {}", line, secret_key);
            secret_key
        }).sum();
        println!("Response Part 1: {}", total_sum);
    } else if part == &2 {
        let mut global_counter: Counter<Vec<i8>> = Counter::new();
        contents.lines().for_each(|line| {
            let mut monkey_prices = HashMap::new();
            let mut secret_key: i64 = line.parse().unwrap();
            let mut unit_values = vec![(secret_key % 10) as u8];
            for i in 0..2000 {
                secret_key = calculate_secret_key(secret_key);
                unit_values.push((secret_key % 10) as u8);
                if i >= 4 {
                    let last_values = &unit_values[i-4..i+1];
                    let last_diffs: Vec<i8> = last_values.windows(2).map(|w| w[1] as i8 - w[0] as i8).collect();
                    if !monkey_prices.contains_key(&last_diffs) {
                        monkey_prices.insert(last_diffs.clone(), last_values[4]);
                        global_counter[&last_diffs] += last_values[4] as usize;
                    }
                    // println!("{:?}: {:?}", last_diffs, last_values[4]);
                }
            }
        });
        println!("Response Part 2: {:?}", global_counter.k_most_common_ordered(1)[0]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mix() {
        let result = mix(42, 15);
        assert_eq!(result, 37);
    }

    #[test]
    fn test_prune() {
        let result = prune(100000000);
        assert_eq!(result, 16113920);
    }

    #[test]
    fn test_calculate_secret_key() {
        let mut result: i64 = 123;
        let expected_results = vec![15887950, 16495136, 527345 ,704524 ,1553684 ,12683156 ,11100544 ,12249484 ,7753432 ,5908254];
        for i in 0..10 {
            result = calculate_secret_key(result);
            println!("{}: {}", i, result);
            assert_eq!(result, expected_results[i]);
        }
    }
}
