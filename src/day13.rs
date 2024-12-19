use regex::Regex;
use nalgebra::{Matrix2, Vector2};

fn get_as_math_problem(lines_iterator: &mut dyn Iterator<Item = &str>, part: &i8) -> (Matrix2<f64>, Vector2<f64>) {
    let re = Regex::new(r"Button (?<btn>[AB])\: X\+(?<x_plus>\d*), Y\+(?<y_plus>\d*)").unwrap();
    let re_prize = Regex::new(r"Prize: X\=(?<x_goal>\d*), Y\=(?<y_goal>\d*)").unwrap();
    let first_line = lines_iterator.next().unwrap();
    let row_vec_a = re.captures_iter(first_line).map(|caps| {
        let x_plus: f64 = caps.name("x_plus").unwrap().as_str().parse().unwrap();
        let y_plus: f64 = caps.name("y_plus").unwrap().as_str().parse().unwrap();
        return Vector2::new(x_plus, y_plus);
    }).next().unwrap();
    let second_line = lines_iterator.next().unwrap();
    let row_vec_b = re.captures_iter(second_line).map(|caps| {
        let x_plus: f64 = caps.name("x_plus").unwrap().as_str().parse().unwrap();
        let y_plus: f64 = caps.name("y_plus").unwrap().as_str().parse().unwrap();
        return Vector2::new(x_plus, y_plus);
    }).next().unwrap();
    let third_line = lines_iterator.next().unwrap();
    let result_vec = re_prize.captures_iter(third_line).map(|caps| {
        let mut x_goal: f64 = caps.name("x_goal").unwrap().as_str().parse().unwrap();
        let mut y_goal: f64 = caps.name("y_goal").unwrap().as_str().parse().unwrap();
        if part == &2 {
            x_goal += 10000000000000.0;
            y_goal += 10000000000000.0;
        }
        return Vector2::new(x_goal, y_goal);
    }).next().unwrap();
    let matrix = Matrix2::from_columns(&[row_vec_a, row_vec_b]);
    return (matrix, result_vec);
}

pub fn run(contents: String, part: &i8) {
    let mut lines_iterator = contents.lines();
    let mut total_cost = 0;
    let cost_vec = Vector2::new(3, 1);
    loop {
        let math_problem = get_as_math_problem(&mut lines_iterator, part);
        let matrix_buttons = math_problem.0;
        let result = math_problem.1;
        // println!("Response: matrix {} result {}", math_problem.0, math_problem.1);
        let solution: Vector2<f64> = matrix_buttons.lu().solve(&result).unwrap();
        if solution.iter().all(|x| (f64::round(*x) - x).abs() < 0.001) {
            let integer_solution = Vector2::new(
                f64::round(solution[0]) as i64,
                f64::round(solution[1]) as i64,
            );
            println!("Solution lu: {:?}", integer_solution);
            let solution_cost = integer_solution.transpose() * cost_vec;
            total_cost += solution_cost[(0, 0)]
        } else {
            println!("Not integer Solution lu: {:?}", solution);
        }
        if None == lines_iterator.next() {
            break;
        }
    }
    println!("Total cost: {}", total_cost);
}