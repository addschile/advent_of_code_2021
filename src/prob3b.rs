use std::{io::{self, BufReader, Read}, fs::File};

#[derive(Debug)]
struct Directions {
    movement: String,
    length: i32
}

impl Directions {

    fn to_point(&self) -> Result<Point, &'static str> {
        match &(self.movement.to_lowercase())[..] {
            "forward" => Ok(Point{vertical: 0, horizontal: self.length, aim: 0}),
            "backward" => Ok(Point{vertical: 0, horizontal: -self.length, aim: 0}),
            "up" => Ok(Point{vertical: 0, horizontal: 0, aim: -self.length}),
            "down" => Ok(Point{vertical: 0, horizontal: 0, aim: self.length}),
            _ => Err("Invalid direction!"),
        }
    }

}

#[derive(Debug)]
struct Point {
    vertical: i32,
    horizontal: i32,
    aim: i32
}

impl Point {

    fn add(&mut self, point: Point) {
        self.aim += point.aim;
        self.horizontal += point.horizontal;
        self.vertical += self.aim*point.horizontal;
    }

    fn step(&mut self, direction: Directions) {
        self.add(direction.to_point().unwrap());
    }

    fn origin() -> Point {
        Point{vertical: 0, horizontal: 0, aim: 0}
    }
}

fn get_test_directions() -> String {
    "forward 5
down 5
forward 8
up 3
down 8
forward 2".to_string()
}

fn get_directions() -> String {
    let mut file = File::open("problem3.txt").unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap().to_string();
    return buf;
}

pub fn problem3() -> io::Result<()> {
    let directions: Vec<Directions> = get_directions().
        lines().
        map(|item| item.split_whitespace().
                        map(|sitem| sitem.to_string()).
                        into_iter().
                        collect::<Vec<String>>()).
        into_iter().
        map(|command| Directions{
                        movement: command[0].clone(),
                        length: command[1].parse::<i32>().unwrap()
                      }).
        collect();
    let mut point = Point::origin();
    for direction in directions.into_iter() {
        point.step(direction);
    }
    println!("Final spot: ({},{}) Multiplied: {}", point.vertical, point.horizontal, point.vertical*point.horizontal);
    Ok(())
}
