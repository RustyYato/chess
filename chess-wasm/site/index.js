let engine = null;
let game = null;

import("./node_modules/chess-wasm/chess_wasm.js").then((js) => {
    js.init_logging();

    if (engine === null) {
        engine = js.new_engine();
    }

    if (game === null) {
        game = js.new_game();
    }

    console.log("search for a move");

    const move = engine.search(game, "100microseconds");

    console.log("found a move: ", move.chess_move());
});
