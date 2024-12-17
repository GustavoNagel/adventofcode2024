use counter::Counter;

fn blink(stone_number: i64) -> Vec<i64> {
    if stone_number == 0 {
        return vec![1];
    } else if stone_number.to_string().len() % 2 == 0 {
        let stone_number_str = stone_number.to_string();
        let half = stone_number_str.len() / 2;
        return vec![stone_number_str[..half].parse().unwrap(), stone_number_str[half..].parse().unwrap()];
    } else {
        return vec![stone_number * 2024];
    }
}

pub fn run(contents: String, part: &i8) {
    let lines: Vec<&str> = contents.lines().collect();
    let x_limit = if part == &1 { 25 } else { 75 };
    // solution with performance issues
    // let mut arranged_stones: Vec<i64> = lines[0].split_whitespace().map(|stone| stone.parse().unwrap()).collect();
    // println!("arranged_stones: {:?}", arranged_stones);
    // for x in 0..x_limit {
    //     println!("iter number: {:}", x + 1);
    //     let blink_result: Vec<Vec<i64>> = arranged_stones.iter().map(|stone| blink(*stone)).collect();
    //     arranged_stones = blink_result.iter().flatten().map(|x| *x).collect();
    //     // println!("new_arranged_stones: {:?}", arranged_stones);
    // }
    // let stones_num = arranged_stones.len();

    // solution with counter
    let arranged_stones: Vec<i64> = lines[0].split_whitespace().map(|stone| stone.parse().unwrap()).collect();
    println!("arranged_stones: {:?}", arranged_stones);
    let mut counter = arranged_stones.iter().map(|x| *x).collect::<Counter<_>>();
    for x in 0..x_limit {
        println!("iter number: {:}", x + 1);
        let mut new_counter: Counter<i64> = Counter::new();
        counter.iter().for_each(|(stone, count)| {
            for i in blink(*stone).into_iter() {
                new_counter[&i] += count;
            }
        });
        counter = new_counter.clone();
    }
    let stones_num: usize = counter.values().map(|count| *count).sum();

    println!("Response: {:?}", stones_num);
}
