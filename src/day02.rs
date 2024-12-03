
fn is_in_range(x: i32) -> bool { x >= 1 && x <= 3 }

pub fn run(contents: String, part: &i8) {
    let mut count_safe_reports: i32 = 0;
    if part == &1 {
        contents.lines().for_each(|line| {
            let report: Vec<i32> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
            if (report[1] > report[0] && report.windows(2).all(|w| w[1] - w[0] <= 3 && w[1] - w[0] >= 1 ))
            || (report[0] > report[1] && report.windows(2).all(|w| w[0] - w[1] <= 3 && w[0] - w[1] >= 1 )){
                count_safe_reports += 1;
            }
        });
        println!("Response: {}", count_safe_reports);
    } else if part == &2 {
        contents.lines().for_each(|line| {
            let report: Vec<i32> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
            let diff_report = report.windows(2).map(|w| w[1] - w[0]);
            let mut diff_vector_report = diff_report.clone().collect::<Vec<i32>>(); 
            let mut error_counter = 0;
            println!("{:?}", diff_report);
            let is_it_growing = diff_report.sum::<i32>() > 0;
            let weird_num = 9999999;
            diff_vector_report.push(weird_num);
            diff_vector_report.insert(0, weird_num);
            for window in diff_vector_report.windows(3) {
                let (mut a, mut b, mut c) = (window[0], window[1], window[2]);
                if !is_it_growing {
                    (a, b, c) = (-a, -b, -c);
                }
                if !is_in_range(b) {
                    println!("{} not in range. Options are sum with {} and {} ", b, a, c);
                    if error_counter >= 1 {
                        println!("--- Count as unsafe: {}", b);
                        return;
                    }
                    error_counter += 1;
                    if (a.abs() == weird_num && is_in_range(c)) || (c.abs() == weird_num && is_in_range(a)) {
                        println!("-We can discard this number: {}", b);
                        continue;
                    } else if a.abs() != weird_num && is_in_range(a+b) && (is_in_range(c) || c.abs() == weird_num) {
                        println!("-We say that a should sum with b : {} + {}", a, b);
                        continue;
                    } else if c.abs() != weird_num && is_in_range(c+b) && (is_in_range(a) || a.abs() == weird_num) {
                        println!("-We say that b should sum with c : {} + {}", b, c);
                        if !is_in_range(c) {
                            // avoid double counting
                            error_counter -= 1;
                        }
                    } else {
                        println!("---Count as unsafe");
                        return;
                    }
                }
            }
            println!("---Count as safe");
            count_safe_reports += 1;
        });
        println!("Response: {}", count_safe_reports);
    }
}
