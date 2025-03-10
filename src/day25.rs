use std::collections::HashSet;

pub fn run(contents: String, part: &i8) {
    let mut locks = vec![];
    let mut keys = vec![];
    let mut lines_iterator = contents.lines();
    while let Some(first_line) = lines_iterator.next(){
        let is_lock = first_line.chars().all(|x| x == '#');
        let mut lock_or_key = vec![0, 0, 0, 0, 0];
        for _ in 0..5 {
            lines_iterator
             .next().unwrap()
             .chars().map(|x| if x == '#' {1} else {0})
             .enumerate().for_each(|(i, x)| lock_or_key[i] += x);
        }
        // println!("{:?}", lock_or_key);
        if is_lock {
            locks.push(lock_or_key);
        } else {
            keys.push(lock_or_key);
        }
        lines_iterator.next();
        lines_iterator.next();
    }
    let mut counter = HashSet::new();
    locks.iter().for_each(|x| {
        keys.iter().for_each(|y| {
            if x.iter().zip(y.iter()).map(|(a, b)| a + b).all(|c| c <= 5) {
                counter.insert(format!("{}{}", x.iter().map(|n| n.to_string()).collect::<String>(), y.iter().map(|n| n.to_string()).collect::<String>()));
            }
        });
    });
    if *part == 1 {
        println!("Response {}", counter.len());
        return;
    } else if *part == 2 {
        println!("Ho ho ho");
    }
}