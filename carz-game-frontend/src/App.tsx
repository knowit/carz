import React from "react";
import useWebSocket from 'react-use-websocket';

import Carz from "./game/Carz"
import { Board, Car, Obstacle, CreateGameMessage } from "./models/interfaces"
import { useKeyPress } from './hooks/useKeyPress';


const App = () => {

  const obstacles: Obstacle[] = [];

  const board: Board = {
    obstacles
  };

  const cars: Car[] = [];

  const socketUrl = "ws://127.0.0.1:6565";

  const {
    sendMessage,
    sendJsonMessage,
    lastMessage,
    lastJsonMessage,
    readyState,
    getWebSocket
  } = useWebSocket(socketUrl, {
    onOpen: () => console.log('Opened connection'),
    shouldReconnect: (closeEvent) => true,
  });

  const keysPressed = useKeyPress();

  const createGameMessage: CreateGameMessage = {
    status: "create_game",
    board,
    cars
  }

  const handleClick = (): void => {
    sendJsonMessage(createGameMessage);
  };

  return(
    <div>
      <button onClick={handleClick}>
        Create Game
      </button>
      <Carz 
        board={board}
        cars={cars}
      />
    </div>
  );

}

export default App;
