pub fn run(contents: String, part: &i8) {
    let positions: Vec<(usize, usize)> = contents.lines().map(|line| {
        let position: (usize, usize) = {
            let mut line_iterator = line.split(",").map(|number| number.parse::<usize>());
            (line_iterator.next().unwrap().unwrap(), line_iterator.next().unwrap().unwrap())
        };
        position
    }).collect();
    println!("Positions: {:?}", positions);
    if part == &1 {
        println!("Response: {}", 0);
    }
}