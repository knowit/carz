use vector2d::Vector2D;

pub mod config;
pub mod car;
pub mod board;
pub mod obstacle;
mod game_statistics;
mod action;

use config::Config;
use car::Car;
use board::Board;
use obstacle::Obstacle;
use game_statistics::GameStatistics;
use game_statistics::GameStatus;
use action::Action;

// TODO: Lag en CACHER som tar vare på initial board?
pub struct Game {
    board: Board,
    game_stats: GameStatistics
}

impl Game {
    pub fn new(board: Board) -> Game {
        Game {
            board: board,
            game_stats: GameStatistics {
                game_status: GameStatus::Playing
            }
        }
    }

    pub fn run(&self) {
        println!("running!");
    }

    pub fn execute_game_loop(&self) {
        
    }
    
    pub fn step(&self, action: Action) {

        println!("We are in step!");

        // get event from user (x and y direction)
        // apply logic
        // check if done (collision or win)
        // send back new car coords
        
    }

    pub fn reset(&self) {
 
    }

    pub fn get_all_actions(&self) -> Vec<String> {
        action::get_all_valid_action_combinations()
    }

    pub fn get_random_action(&self) {

    }
}

