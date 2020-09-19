use vector2d::Vector2D;
use super::car::Car;
use super::obstacle::Obstacle;

pub struct Board {
    length: i32,
    height: i32,
    cars: Vec<Car>,
    obstacles: Vec<Obstacle>,
    goal: Vector2D<i32>
}

impl Board {
    // TODO: Make this a defualt profile
    pub fn new(cars: Vec<Car>, obstacles: Vec<Obstacle>) -> Board {
        Board {
            length: 800,
            height: 800, 
            cars: cars,
            obstacles: obstacles,
            goal: Vector2D::new(700, 700)
        }
    }
}