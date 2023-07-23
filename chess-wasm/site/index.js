import("./node_modules/chess-wasm/chess_wasm.js").then((js) => {
    js.greet("WebAssembly with npm");
});
