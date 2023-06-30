import init, { World, Direction, InitOutput } from 'snake_game';

init().then((wasm) => {
  const CELL_SIZE = 24;
  const WORLD_WIDTH = 32;
  const snakeSpawnIndex = Date.now() % (WORLD_WIDTH * WORLD_WIDTH);

  const world = World.from(WORLD_WIDTH, snakeSpawnIndex);
  const canvas = <HTMLCanvasElement>document.getElementById('snake-canvas');
  const ctx = canvas.getContext('2d');

  const worldWidth = world.width();

  canvas.height = worldWidth * CELL_SIZE;
  canvas.width = worldWidth * CELL_SIZE;

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

    ctx.stroke();
  };

  const drawSnake = () => {
    const snakeCells = getSnakeCells(wasm, world);

    snakeCells.forEach((cellIdx, i) => {
      const col = cellIdx % worldWidth;
      const row = Math.floor(cellIdx / worldWidth);

      ctx.fillStyle = i === 0 ? '#7878db' : '#333';

      ctx.beginPath();
      ctx.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);
    });

    ctx.stroke();
  };

  const paint = () => {
    drawWorld();
    drawSnake();
  };

  const update = () => {
    const fps = 10;
    setTimeout(() => {
      ctx.clearRect(0, 0, canvas.width, canvas.height);

      world.step();
      paint();

      requestAnimationFrame(update);
    }, 1000 / fps);
  };

  paint();
  update();
});

const getSnakeCells = (wasm: InitOutput, world: World): Uint32Array => {
  const snakeCellPtr = world.snake_cells();
  const snakeLen = world.snake_length();

  return new Uint32Array(wasm.memory.buffer, snakeCellPtr, snakeLen);
};
