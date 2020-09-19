import { useState, useEffect } from "react";

export const useKeyPress = () => {
  const [keysPressed, setKeyPressed] = useState(new Set([]));

  const isLegal = (key: string): boolean => {
      const validKeys = ["ArrowLeft", "ArrowRight", "ArrowUp", "ArrowDown"]
      if (validKeys.includes(key)) {
          return true;
      }
      return false;
  }

  const downHandler = ({ key } : { key: string }) => {
      if (isLegal(key)) {
        setKeyPressed(keysPressed.add(key));
      }
  }

  const upHandler = ({ key } : { key: string }) => {
      if (isLegal(key)) {
        keysPressed.delete(key);
        setKeyPressed(keysPressed);
      }
  };

  useEffect(() => {
    window.addEventListener("keydown", downHandler);
    window.addEventListener("keyup", upHandler);
    return () => {
      window.removeEventListener("keydown", downHandler);
      window.removeEventListener("keyup", upHandler);
    };
  }, []); // Empty array ensures that effect is only run on mount and unmount

  return keysPressed;
}

