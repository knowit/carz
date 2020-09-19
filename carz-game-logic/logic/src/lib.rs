mod game;

use game::Game;
use game::car::Car;
use game::car::Control;
use game::obstacle::Obstacle;
use game::board::Board;
use std::fs;

pub fn init_game() -> Game {
    let file = fs::read_to_string("logic/config.yml").unwrap();

    Game::new(
        init_board(
            init_cars(file), 
            init_obstacles()
        )
    )
}


// TODO: Should these functions be here?
fn init_board(cars: Vec<Car>, obstacles: Vec<Obstacle>) -> Board {
    Board::new(cars, obstacles)
}

fn init_cars(file: String) -> Vec<Car> {

    // need a separate config for each car. 1: problem with ownership. 2: each car not necessarily the same
    // TODO: Get number_of_cars_from somewhere else
    let number_of_cars = 2;
    let mut cars: Vec<Car> = Vec::new();

    for x in 0..number_of_cars {
        let config = serde_yaml::from_str(&file).unwrap();
        let control: Control = if x%1 == 0 { Control::HUMAN } else { Control::AI };
        cars.push(Car::new_default(config, control, x));
    }

    cars

}

fn init_obstacles() -> Vec<Obstacle> {
    let mut obstacles: Vec<Obstacle> = Vec::new();

    obstacles
}