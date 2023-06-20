use std::fs::File;
use std::io::Write;

use postflop_solver::{
    PostFlopGame, NOT_DEALT,
    card_to_string, card_from_str, holes_to_strings,
};


fn round_values(values: Vec<f32>, places: u32) -> Vec<f32> {
    values
        .iter()
        .map(|&value| (value * 10_f32.powi(places as i32)).round() / 10_f32.powi(places as i32))
        .collect()
}


pub struct Exporter<'a> {
    pub game: &'a mut PostFlopGame,
    pub cards: Vec<&'static str>,
    file: File,
}

impl<'a> Exporter<'a> {
    pub fn new(game: &'a mut PostFlopGame, cards: Vec<&'static str>) -> Result<Self, std::io::Error> {
        let file = File::create("result.txt")?;
        Ok(Exporter {
            game,
            cards,
            file,
        })
    }

    fn print<T: std::fmt::Debug>(&mut self, arg: T, indent: usize) {
        let indent_str: String = (0..indent).map(|_| '\t').collect();
        let to_print = format!("{:?}", arg);

        if let Err(err) = self.file.write_all(format!("{}{}\n", indent_str, to_print).as_bytes()) {
            eprintln!("Error writing to file: {}", err);
        }
    }

    fn get_board(&mut self) -> Result<String, String> {
        let config = self.game.card_config();
        let raw = [
            config.flop[2],
            config.flop[1],
            config.flop[0],
            config.turn,
            config.river,
        ];

        let board_str: Vec<String> = raw.iter()
            .filter(|&card| *card != NOT_DEALT)
            .map(|&card| card_to_string(card))
            .collect::<Result<Vec<String>, _>>()?;

        Ok(board_str.join(","))
    }

    pub fn export_init(&mut self) {
        let tree_config = self.game.tree_config();
        self.print([tree_config.starting_pot, tree_config.effective_stack], 0);

        let board = self.get_board().unwrap();
        self.print(board, 0);

        let oop_cards = self.game.private_cards(0);
        let ip_cards = self.game.private_cards(1);

        let oop_cards_str = holes_to_strings(oop_cards).unwrap();
        let ip_cards_str = holes_to_strings(ip_cards).unwrap();

        self.print(oop_cards_str, 0);
        self.print(ip_cards_str, 0);
    }

    pub fn export(&mut self, indent: usize) {
        if self.game.is_terminal_node() {
            return;
        }

        self.print(self.game.current_player(), indent);
        if self.game.is_chance_node() {
            self.export_chance(indent);
        } else {
            self.export_player(indent);
        }
    }

    fn export_chance(&mut self, indent: usize) {
        let history = self.game.history().to_owned();

        for card in self.cards.to_owned() {
            let card_index = card_from_str(card).unwrap();
            if self.game.possible_cards() & (1 << card_index) == 0 {
                // Card not available
                continue;
            }

            self.print(card, indent);
            self.export_child(card_index as usize, indent);
            self.game.apply_history(&history);
        }
    }

    fn export_player(&mut self, indent: usize) {
        let history = self.game.history().to_owned();
        let current_player = self.game.current_player();

        self.game.cache_normalized_weights();
        let strategy = self.game.strategy();
        let evs = self.game.expected_values_detail(current_player);

        self.print(round_values(strategy, 3), indent);
        self.print(round_values(evs, 0), indent);

        for (index, action) in self.game.available_actions().iter().enumerate() {
            self.print(format!("{:?}", action), indent);
            self.export_child(index, indent);
            self.game.apply_history(&history);
        }
    }

    fn export_child(&mut self, action: usize, indent: usize) {
        self.game.play(action);
        self.export(indent + 1);
    }
}
