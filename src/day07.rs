
fn check_test_value(test_value: i64, inputs: Vec<i64>, part: &i8) -> bool {
    if inputs.len() == 1 {
        return inputs[0] == test_value;
    }
    let mut inputs_clone = inputs.clone();
    let initial_value = test_value.clone();
    let last_value = inputs_clone.pop().unwrap();
    if initial_value % last_value == 0 && check_test_value(initial_value / last_value, inputs_clone.clone(), part){
        return true;
    }
    if initial_value - last_value > 0 && check_test_value(initial_value - last_value, inputs_clone.clone(), part){
        return true;
    }
    if part == &2 {
        let initial_value_as_str = initial_value.to_string();
        let last_value_as_str = last_value.to_string();
        let diff_len = (initial_value_as_str.len() as isize - last_value_as_str.len() as isize) as usize;
        if diff_len > 0 && initial_value_as_str.ends_with(&last_value_as_str) {
            let new_initial_value = initial_value_as_str[..diff_len].parse().unwrap();
            if check_test_value(new_initial_value, inputs_clone.clone(), part) {
                return true;
            }
        }
    }
    false
}

pub fn run(contents: String, part: &i8) {
    let mut test_values_sum: i64 = 0;
    contents.lines().for_each(|line| {
        let (test_value, inputs): (i64, Vec<i64>) = {
            let mut parts = line.split(": ");
            (parts.next().unwrap().parse().unwrap(), parts.next().unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect())
        };
        if check_test_value(test_value, inputs, part) {
            test_values_sum += test_value;
        }
    });
    println!("Response: {}", test_values_sum);
}