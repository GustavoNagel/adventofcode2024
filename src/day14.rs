use regex::Regex;
use counter::Counter;

use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy)]
struct Velocity {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy)]
struct Robot {
    position: Position,
    velocity: Velocity,
    space_size: (i32, i32),
}

impl Robot {
    fn move_robot(&mut self, time: i32) {
        let new_x = self.position.x + self.velocity.x * time;
        let new_y = self.position.y + self.velocity.y * time;
        self.position.x = if new_x > 0 { new_x % self.space_size.0} else {(self.space_size.0 - (- new_x % self.space_size.0)) % self.space_size.0 };
        self.position.y = if new_y > 0 { new_y % self.space_size.1} else {(self.space_size.1 - (- new_y % self.space_size.1)) % self.space_size.1 };
    }
}

fn show_robots(robots: &Vec<Robot>, space_size: (i32, i32), i: i32) {
    let reset_color = "\x1b[0m";
    let green_hashtag = "\x1b[48;5;46m#\x1b[0m";
    let mut space = vec![vec!["."; space_size.0 as usize]; space_size.1 as usize];
    robots.iter().for_each(|robot| {
        let Position { x, y } = robot.position;
        space[y as usize][x as usize] = green_hashtag;
    });
    if space.iter().any(|line| {
        // count how many green hashtags are in the line sequentially without interruptions
        line.into_iter().chunk_by(|&x| *x == green_hashtag).into_iter().any(|(is_green, group)| {
            is_green && group.count() > 10
        })
    }) {
        println!("Iteration: {}", i);
        space.iter().for_each(|line| {
            println!("{}.{}", line.join(""), reset_color);
        });
    }
}

pub fn run(contents: String, part: &i8) {
    let re = Regex::new(r"p=(?<p_x>\-?\d*),(?<p_y>\-?\d*) v=(?<v_x>\-?\d*),(?<v_y>\-?\d*)").unwrap();
    let entries = contents.lines().map(|line| {
        let caps = re.captures(line).unwrap();
        let p_x: i32 = caps.name("p_x").unwrap().as_str().parse().unwrap();
        let p_y: i32 = caps.name("p_y").unwrap().as_str().parse().unwrap();
        let v_x: i32 = caps.name("v_x").unwrap().as_str().parse().unwrap();
        let v_y: i32 = caps.name("v_y").unwrap().as_str().parse().unwrap();
        println!("{:?}", (Position { x: p_x, y: p_y }, Velocity { x: v_x, y: v_y }));
        (Position { x: p_x, y: p_y }, Velocity { x: v_x, y: v_y })
    }).collect::<Vec<(Position,Velocity)>>();
    // println!("Entries: {:?}", entries);

    let space_size = (101, 103);
    let robots = entries.iter().map(|(robot_position, robot_velocity)| {
        let robot = Robot {
            position: *robot_position,
            velocity: *robot_velocity,
            space_size: space_size,
        };
        // println!("{:?}", robot.position);
        robot
    }).collect::<Vec<Robot>>();

    if part == &1 {
        let moved_robots = robots.iter().map(|robot| {
            let mut robot = *robot;
            robot.move_robot(100);
            robot
        }).collect::<Vec<Robot>>();
        let mut quadrants_counter: Counter<i32> = Counter::new();
        let quadrant_limit = (space_size.0 / 2, space_size.1 / 2);
        println!("{:?}", quadrant_limit);
        moved_robots.iter().for_each(|robot| {
            let Position { x, y } = robot.position;
            if x < quadrant_limit.0 && y < quadrant_limit.1 {
                quadrants_counter[&1] += 1;
            } else if x > quadrant_limit.0 && y < quadrant_limit.1 {
                quadrants_counter[&2] += 1;
            } else if x < quadrant_limit.0 && y > quadrant_limit.1 {
                quadrants_counter[&3] += 1;
            } else if x > quadrant_limit.0 && y > quadrant_limit.1 {
                quadrants_counter[&4] += 1;
            }
            println!("{:?}", robot.position);
        });
        println!("Counter {:?}", quadrants_counter);
        println!("Response: {}", quadrants_counter.values().map(|&v| v as i32).reduce(|acc, e| acc * e).unwrap());
    } else {
        let mut moved_robots = robots.clone();
        for i in 1.. {
            moved_robots = moved_robots.iter().map(|robot| {
                let mut robot = *robot;
                robot.move_robot(1);
                robot
            }).collect::<Vec<Robot>>();
            show_robots(&moved_robots, space_size, i);
        }
    }
}