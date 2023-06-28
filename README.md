# PokerTrainer
Practice using strategy files generated with an engine extended from [postflop-solver](https://github.com/b-inary/postflop-solver).

## 1. Generate training spots

- Go to [rust](/rust) folder
- Copy [main_default.rs](/rust/src/main_default.rs) to `main.rs` and edit the game settings
- Run `cargo run --release`

If successful, you should get a "result.txt" solution that you can move to the [Spots](/Spots) folder.

## 2. Practice

Run `python main.py`.

The trainer will look for solutions in the [Spots](/Spots) folder,
or use [toy_game_AKK7.txt](/tests/fixtures/toy_game_AKK7.txt) as default.
