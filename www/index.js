import init, { World } from 'snake_game';

await init();

const world = World.new();
console.log(world.width());
