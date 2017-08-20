// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot{
    x: isize,
    y: isize,
    d: Direction,
}

impl Robot {
    pub fn new(x: isize, y: isize, d: Direction) -> Self {
        Robot {
            x: x,
            y: y,
            d: d,
        }
    }

    pub fn turn_right(&self) -> Self {
        let direction = match self.d {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
        Self {
            x: self.x,
            y: self.y,
            d: direction
        }
    }

    pub fn turn_left(&mut self) -> Self {
        let direction = match self.d {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        };
        Self {
            x: self.x,
            y: self.y,
            d: direction
        }
    }

    pub fn advance(&self) -> Self {
        let mut robot = Robot::new(self.x, self.y, self.d.clone());
        match self.d {
            Direction::North => robot.y += 1,
            Direction::East => robot.x += 1,
            Direction::South => robot.y -= 1,
            Direction::West => robot.x -= 1,
        };
        robot
    }

    pub fn instructions(&self, instructions: &str) -> Self {
        let mut robot = Robot::new(self.x, self.y, self.d.clone());
        for c in instructions.chars() {
            robot = match c {
                'R' => robot.turn_right(),
                'L' => robot.turn_left(),
                'A' => robot.advance(),
                _ => robot,
            };
        }
        robot
    }

    pub fn position(&self) -> (isize, isize) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}

impl PartialEq for Robot {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.d == other.d
    }
}