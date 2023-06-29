async function init() {
  const importObject = {
    console: {
      log: () => {
        console.log('Just logging something');
      },
      error: () => {
        console.log("I'm just an error");
      },
    },
  };

  const res = await fetch('sum.wasm');
  const buffer = await res.arrayBuffer();
  const wasm = await WebAssembly.instantiate(buffer, importObject);

  const sumFunction = wasm.instance.exports.sum;
  const result = sumFunction(70, 80);
  console.log(result);
}

init();
