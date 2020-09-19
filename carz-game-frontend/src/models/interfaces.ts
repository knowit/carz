
export interface Board {
    obstacles: Obstacle[];
}

export interface Obstacle {
    position: Position;
    angle: number;
}

export interface Car {
    id: number;
    position: Position;
    angle: number;
}

export interface Goal {
    position: Position;
}

export interface Position {
    x: number;
    y: number;
}

export interface CreateGameMessage {
    status: string;
    board: Board;
    cars: Car[];
}
