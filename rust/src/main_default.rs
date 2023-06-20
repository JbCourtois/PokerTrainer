use std::convert::TryFrom;

use postflop_solver::*;

mod exporter;
use exporter::Exporter;


fn main() {
    let mut game = generate_game();

    // solve the game
    game.allocate_memory(false);
    let max_num_iterations = 1000;
    let target_exploitability = game.tree_config().starting_pot as f32 * 0.005; // 0.5% of the pot

    solve(&mut game, max_num_iterations, target_exploitability, true);

    let mut exporter = Exporter::new(
        &mut game,
        vec!["Ad", "Qh", "7s", "6d", "4h", "2c"],
    ).unwrap();

    exporter.export_init();
    exporter.export(0);
}

fn generate_game() -> PostFlopGame {
    // ranges of OOP and IP in string format
    let oop_range = "KK,3s2s,3h2h,3d2d";
    let ip_range = "AJs";

    let card_config = CardConfig {
        range: [oop_range.parse().unwrap(), ip_range.parse().unwrap()],
        flop: flop_from_str("AhKdKc").unwrap(),
        turn: card_from_str("7c").unwrap(),
        // turn: NOT_DEALT,
        river: NOT_DEALT,
    };

    // bet sizes -> 60% of the pot, geometric size, and all-in
    // raise sizes -> 2.5x of the previous bet
    // see the documentation of `BetSizeCandidates` for more details
    let bet_sizes = BetSizeCandidates::try_from(("30%, 100%, a", "60%, a")).unwrap();

    let expected_state = match (card_config.turn != NOT_DEALT, card_config.river != NOT_DEALT) {
        (false, _) => BoardState::Flop,
        (true, false) => BoardState::Turn,
        (true, true) => BoardState::River,
    };

    let tree_config = TreeConfig {
        initial_state: expected_state,
        starting_pot: 600,
        effective_stack: 2000,
        rake_rate: 0.0,
        rake_cap: 0.0,
        flop_bet_sizes: [bet_sizes.clone(), bet_sizes.clone()], // [OOP, IP]
        turn_bet_sizes: [bet_sizes.clone(), bet_sizes.clone()],
        river_bet_sizes: [bet_sizes.clone(), bet_sizes.clone()],
        turn_donk_sizes: None, // use default bet sizes
        river_donk_sizes: None,
        add_allin_threshold: 1.5, // add all-in if (maximum bet size) <= 1.5x pot
        force_allin_threshold: 0.15, // force all-in if (SPR after the opponent's call) <= 0.15
        merging_threshold: 0.1,
    };

    // build the game tree
    let action_tree = ActionTree::new(tree_config).unwrap();
    PostFlopGame::with_config(card_config, action_tree).unwrap()
}
