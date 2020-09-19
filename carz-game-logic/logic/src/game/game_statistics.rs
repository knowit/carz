pub struct GameStatistics {
    pub game_status: GameStatus
}

pub enum GameStatus {
    Win,
    Lose,
    Playing
}