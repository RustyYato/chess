use chess_movegen::Board;

fn movegen_perft_test(fen: &str, depth: usize, result: usize) {
    let board: Board = fen.parse().unwrap();

    assert_eq!(board.perft_test(depth), result);
    // assert_eq!(MoveGen::movegen_perft_test_piecewise(&board, depth), result);
}

#[test]
fn movegen_perft_kiwipete() {
    movegen_perft_test(
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
        5,
        193690690,
    );
}

#[test]
fn movegen_perft_1() {
    movegen_perft_test("8/5bk1/8/2Pp4/8/1K6/8/8 w - d6 0 1", 6, 824064);
    // Invalid FEN
}

#[test]
fn movegen_perft_2() {
    movegen_perft_test("8/8/1k6/8/2pP4/8/5BK1/8 b - d3 0 1", 6, 824064);
    // Invalid FEN
}

#[test]
fn movegen_perft_3() {
    movegen_perft_test("8/8/1k6/2b5/2pP4/8/5K2/8 b - d3 0 1", 6, 1440467);
}

#[test]
fn movegen_perft_4() {
    movegen_perft_test("8/5k2/8/2Pp4/2B5/1K6/8/8 w - d6 0 1", 6, 1440467);
}

#[test]
fn movegen_perft_5() {
    movegen_perft_test("5k2/8/8/8/8/8/8/4K2R w K - 0 1", 6, 661072);
}

#[test]
fn movegen_perft_6() {
    movegen_perft_test("4k2r/8/8/8/8/8/8/5K2 b k - 0 1", 6, 661072);
}

#[test]
fn movegen_perft_7() {
    movegen_perft_test("3k4/8/8/8/8/8/8/R3K3 w Q - 0 1", 6, 803711);
}

#[test]
fn movegen_perft_8() {
    movegen_perft_test("r3k3/8/8/8/8/8/8/3K4 b q - 0 1", 6, 803711);
}

#[test]
fn movegen_perft_9() {
    movegen_perft_test("r3k2r/1b4bq/8/8/8/8/7B/R3K2R w KQkq - 0 1", 4, 1274206);
}

#[test]
fn movegen_perft_10() {
    movegen_perft_test("r3k2r/7b/8/8/8/8/1B4BQ/R3K2R b KQkq - 0 1", 4, 1274206);
}

#[test]
fn movegen_perft_11() {
    movegen_perft_test("r3k2r/8/3Q4/8/8/5q2/8/R3K2R b KQkq - 0 1", 4, 1720476);
}

#[test]
fn movegen_perft_12() {
    movegen_perft_test("r3k2r/8/5Q2/8/8/3q4/8/R3K2R w KQkq - 0 1", 4, 1720476);
}

#[test]
fn movegen_perft_13() {
    movegen_perft_test("2K2r2/4P3/8/8/8/8/8/3k4 w - - 0 1", 6, 3821001);
}

#[test]
fn movegen_perft_14() {
    movegen_perft_test("3K4/8/8/8/8/8/4p3/2k2R2 b - - 0 1", 6, 3821001);
}

#[test]
fn movegen_perft_15() {
    movegen_perft_test("8/8/1P2K3/8/2n5/1q6/8/5k2 b - - 0 1", 5, 1004658);
}

#[test]
fn movegen_perft_16() {
    movegen_perft_test("5K2/8/1Q6/2N5/8/1p2k3/8/8 w - - 0 1", 5, 1004658);
}

#[test]
fn movegen_perft_17() {
    movegen_perft_test("4k3/1P6/8/8/8/8/K7/8 w - - 0 1", 6, 217342);
}

#[test]
fn movegen_perft_18() {
    movegen_perft_test("8/k7/8/8/8/8/1p6/4K3 b - - 0 1", 6, 217342);
}

#[test]
fn movegen_perft_19() {
    movegen_perft_test("8/P1k5/K7/8/8/8/8/8 w - - 0 1", 6, 92683);
}

#[test]
fn movegen_perft_20() {
    movegen_perft_test("8/8/8/8/8/k7/p1K5/8 b - - 0 1", 6, 92683);
}

#[test]
fn movegen_perft_21() {
    movegen_perft_test("K1k5/8/P7/8/8/8/8/8 w - - 0 1", 6, 2217);
}

#[test]
fn movegen_perft_22() {
    movegen_perft_test("8/8/8/8/8/p7/8/k1K5 b - - 0 1", 6, 2217);
}

#[test]
fn movegen_perft_23() {
    movegen_perft_test("8/k1P5/8/1K6/8/8/8/8 w - - 0 1", 7, 567584);
}

#[test]
fn movegen_perft_24() {
    movegen_perft_test("8/8/8/8/1k6/8/K1p5/8 b - - 0 1", 7, 567584);
}

#[test]
fn movegen_perft_25() {
    movegen_perft_test("8/8/2k5/5q2/5n2/8/5K2/8 b - - 0 1", 4, 23527);
}

#[test]
fn movegen_perft_26() {
    movegen_perft_test("8/5k2/8/5N2/5Q2/2K5/8/8 w - - 0 1", 4, 23527);
}

#[test]
fn movegen_perft_27() {
    movegen_perft_test(
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 0",
        4,
        197281,
    );
}

#[test]
fn movegen_perft_28() {
    movegen_perft_test(
        "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
        5,
        164_075_551,
    )
}

#[test]
fn movegen_issue_15() {
    let board: Board = "rnbqkbnr/ppp2pp1/4p3/3N4/3PpPp1/8/PPP3PP/R1B1KBNR b KQkq f3 0 1"
        .parse()
        .unwrap();
    let _ = board.legals();
}
