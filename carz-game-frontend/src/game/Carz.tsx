import React from "react";
import { css } from 'emotion';
import { Board, Car } from '../models/interfaces';

const csstest = 800;

const test = css`
    margin: 10px;
    display: grid;
    grid-template-columns: ${csstest}px ${csstest}px;
    grid-template-rows: ${csstest}px;
    grid-gap: 10px;
    background-color: #fff;
    color: #444;

    .doubletest {
        background-color: #000;
    }
`;

interface CarzProps {
  board: Board;
  cars: Car[];
}

const Carz = (props: CarzProps) => {

  const handleRestart = (): void => {
    console.log("test")
  };
    
  return(
    <div className={test}>
        <canvas id="canvas">

        </canvas>
        <div className="doubletest">
          Molly
        </div>
        <button onClick={handleRestart}>
          Restart
        </button>
    </div>
  );
}

export default Carz;