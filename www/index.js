import initSnakeGame, { greet } from 'snake_game';

const wasm = await initSnakeGame();
console.log(wasm);
greet('Martin');
