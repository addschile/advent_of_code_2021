use std::{io::{self, BufReader, Read}, fs::File};

#[derive(Debug)]
struct Directions {
    movement: String,
    length: i32
}

impl Directions {

    fn to_point(&self) -> Result<Point, &'static str> {
        match &(self.movement.to_lowercase())[..] {
            "forward" => Ok(Point{vertical: 0, horizontal: self.length}),
            "backward" => Ok(Point{vertical: 0, horizontal: -self.length}),
            "up" => Ok(Point{vertical: -self.length, horizontal: 0}),
            "down" => Ok(Point{vertical: self.length, horizontal: 0}),
            _ => Err("Invalid direction!"),
        }
    }

}

#[derive(Debug)]
struct Rates {
    gamma: i32,
    epsilon: i32
}

impl Point {

    fn add(&mut self, point: Point) {
        self.vertical += point.vertical;
        self.horizontal += point.horizontal;
    }

    fn step(&mut self, direction: Directions) {
        self.add(direction.to_point().unwrap());
    }

    fn origin() -> Point {
        Point{vertical: 0, horizontal: 0}
    }
}

fn get_test() -> String {
    "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010".to_string()
}

fn get_input() -> String {
    let mut file = File::open("problem4.txt").unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap().to_string();
    return buf;
}

pub fn problem3() -> io::Result<()> {
    let codes: Vec<String> = get_test().
        lines().
        map(|item| item.to_string()).
        collect();
    let vec = vec![0; 5];
    for code in codes {
        for (idx, digit) in code.into_iter().enumerate() {
            let num: i32 = parse<i32>(digit);
            counter[idx] += 2*num - 1;
        }
    }
//        map(|item| item.split_whitespace().
//                        map(|sitem| sitem.to_string()).
//                        into_iter().
//                        collect::<Vec<String>>()).
//        into_iter().
//        map(|command| Directions{
//                        movement: command[0].clone(),
//                        length: command[1].parse::<i32>().unwrap()
//                      }).
//        collect();
//    let mut point = Point::origin();
//    for direction in directions.into_iter() {
//        point.step(direction);
//    }
//    println!("Final spot: ({},{}) Multiplied: {}", point.vertical, point.horizontal, point.vertical*point.horizontal);
    Ok(())
}
