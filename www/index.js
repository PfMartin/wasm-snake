import init, { greet } from 'snake_game';

init().then((wasm) => {
  greet('Martin');
});
