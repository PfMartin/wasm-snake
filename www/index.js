async function init() {
  const res = await fetch('sum.wasm');
  const buffer = await res.arrayBuffer();
  const wasm = await WebAssembly.instantiate(buffer);

  const sumFunction = wasm.instance.exports.sum;
  const result = sumFunction(70, 80);
  console.log(result);
}

init();
