import init, { World, Direction, InitOutput, GameStatus } from 'snake_game';
import { random } from './utils/rnd';

init().then((wasm) => {
  const CELL_SIZE = 10;
  const WORLD_WIDTH = 50;
  const FPS = 20;

  const snakeSpawnIndex = random(WORLD_WIDTH * WORLD_WIDTH);
  const world = World.from(WORLD_WIDTH, snakeSpawnIndex);

  const points = document.getElementById('points');
  const gameStatus = document.getElementById('game-status');
  const gameControlButton = document.getElementById('game-control-btn');
  const canvas = <HTMLCanvasElement>document.getElementById('snake-canvas');
  const ctx = canvas.getContext('2d');

  const worldWidth = world.width();

  canvas.height = worldWidth * CELL_SIZE;
  canvas.width = worldWidth * CELL_SIZE;

  gameControlButton.addEventListener('click', () => {
    const status = world.game_status();

    if (status === undefined) {
      gameControlButton.textContent = 'Restart';
      world.start_game();
      play();
    } else {
      location.reload();
    }
  });

  document.addEventListener('keydown', (e) => {
    switch (e.code) {
      case 'ArrowUp':
        world.change_snake_dir(Direction.Up);
        break;
      case 'ArrowRight':
        world.change_snake_dir(Direction.Right);
        break;
      case 'ArrowDown':
        world.change_snake_dir(Direction.Down);
        break;
      case 'ArrowLeft':
        world.change_snake_dir(Direction.Left);
        break;
    }
  });

  const drawWorld = () => {
    ctx.beginPath();

    for (let x = 0; x < worldWidth + 1; x++) {
      ctx.moveTo(CELL_SIZE * x, 0);
      ctx.lineTo(CELL_SIZE * x, worldWidth * CELL_SIZE);
    }

    for (let y = 0; y < worldWidth + 1; y++) {
      ctx.moveTo(0, CELL_SIZE * y);
      ctx.lineTo(worldWidth * CELL_SIZE, CELL_SIZE * y);
    }
  };

  const drawSnake = () => {
    const snakeCells = getSnakeCells(wasm, world);

    snakeCells
      .filter((cellIdx, i) => i === 0 || cellIdx !== snakeCells[0])
      .forEach((cellIdx, i) => {
        const col = cellIdx % worldWidth;
        const row = Math.floor(cellIdx / worldWidth);

        ctx.fillStyle = i === 0 ? '#7878db' : '#333';

        ctx.beginPath();
        ctx.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);
      });

    ctx.stroke();
  };

  const drawReward = () => {
    const idx = world.reward_cell();
    const col = idx % worldWidth;
    const row = Math.floor(idx / worldWidth);

    ctx.beginPath();
    ctx.fillStyle = '#FF0000';
    ctx.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);
    ctx.stroke();
  };

  const drawGameStatus = () => {
    gameStatus.textContent = world.game_status_text();
    points.textContent = `${world.points()}`;
  };

  const paint = () => {
    drawWorld();
    drawSnake();
    drawReward();
    drawGameStatus();
  };

  const play = () => {
    const status = world.game_status();
    if (status === GameStatus.Won || status === GameStatus.Lost) {
      gameControlButton.textContent = 'Play again';
    }

    setTimeout(() => {
      ctx.clearRect(0, 0, canvas.width, canvas.height);

      world.step();
      paint();

      requestAnimationFrame(play);
    }, 1000 / FPS);
  };

  paint();
});

const getSnakeCells = (wasm: InitOutput, world: World): Uint32Array => {
  const snakeCellPtr = world.snake_cells();
  const snakeLen = world.snake_length();

  return new Uint32Array(wasm.memory.buffer, snakeCellPtr, snakeLen);
};
