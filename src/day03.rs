use regex::Regex;

pub fn run(contents: String, part: &i8) {
    let re = Regex::new(r"mul\((?<a>\d{1,3}),(?<b>\d{1,3})\)").unwrap();
    if part == &1 {
        let final_results = contents.lines().map(|line| {
            let line_results = re.captures_iter(line).map(|caps| {
                let a: i32 = caps.name("a").unwrap().as_str().parse().unwrap();
                let b: i32 = caps.name("b").unwrap().as_str().parse().unwrap();
                a * b
            });
            line_results.sum::<i32>()
        });
        println!("Response: {}", final_results.sum::<i32>());

    } else if part == &2 {
        let re = Regex::new(r"(?<instruction>mul|don\'t|do)\((?<params>\d{0,3},?\d{0,3})\)").unwrap();
        let final_results = contents.lines().map(|line| {
            let mut enabled = true;
            let mut line_results_sum = 0;
            for caps in re.captures_iter(line) {
                let instruction = caps.name("instruction").unwrap().as_str();
                let params = caps.name("params").unwrap().as_str();
                println!("{} {}", instruction, params);
                match instruction {
                    "mul" => {
                        if enabled {
                          let splitted: Vec<&str> = params.split(",").collect();
                          if splitted.len() == 2 {
                              let a: i32 = splitted[0].parse().unwrap();
                              let b: i32 = splitted[1].parse().unwrap();
                              line_results_sum += a * b;
                            //   println!("{} x {} = {}", a, b, a * b);
                          }
                        //   println!("{:?}", splitted);
                        }
                    }
                    "don't" => {
                        if params.is_empty() {enabled = false;}
                    }
                    "do" => {
                        if params.is_empty() {enabled = true;}
                    }
                    _ => {}
                }
            }
            line_results_sum
        });
        println!("Response: {}", final_results.sum::<i32>());
    }
}
