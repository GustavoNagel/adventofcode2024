
fn get_sorted_vectors(contents: String) -> (Vec<i32>, Vec<i32>) {
    let mut v_left: Vec<i32> = Vec::new();
    let mut v_right: Vec<i32> = Vec::new();
    contents.lines().for_each(|line| {
        let v: Vec<&str> = line.split_whitespace().collect();
        let a: i32 = v[0].parse().unwrap();
        let b: i32 = v[1].parse().unwrap();
        v_left.push(a);
        v_right.push(b);
    });
    v_left.sort();
    v_right.sort();
    (v_left, v_right)
}

pub fn run(contents: String, part: &i8) {
    let mut sum_value: i32 = 0;
    let (v_left, v_right) = get_sorted_vectors(contents);
    if part == &1 {
        let it = v_left.iter().zip(v_right.iter());
        it.for_each(|(a, b)| {
            sum_value += if a > b { a - b } else { b - a };
        });
        println!("Response: {}", sum_value);

    } else if part == &2 {
        let it_left = v_left.iter();
        let mut it_right = v_right.iter();
        let mut num2 = it_right.next().unwrap();
        let mut last_num = 0;
        let mut counter = 0;
        for num in it_left {
            if last_num == *num {
                sum_value += counter * num;
                continue;
            }
            counter = 0;
            while num2 <= num {
                if num2 == num {
                    counter += 1;
                }
                match it_right.next() {
                    Some(n) => num2 = n,
                    None => break,
                }
            }
            last_num = *num;
            sum_value += counter * num;

        }
        println!("Response: {}", sum_value);
    }
}
