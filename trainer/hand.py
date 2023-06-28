import os
from random import random, shuffle

from tabulate import tabulate

from .cards import Card, CardSet
from .logger import LoggerMixin


class Hand(LoggerMixin):
    CHANCE_ID = 2

    def __init__(self, game, human_id, *args, **kwargs):
        super().__init__(*args, **kwargs)

        self.human_id = human_id
        self.node = game.tree
        self._init_cards(game.ranges)
        self.board = CardSet.from_cards(game.board)

        self.history = []
        self.info = []

        self.log('Board:', self.board)
        self.log('Pot:', game.pot)
        self.log()

    def _init_cards(self, ranges):
        self.total_holes = [len(rrr) for rrr in ranges]

        oop_index = int(random() * self.total_holes[0])
        oop_hole = ranges[0][oop_index]
        oop_hole = [oop_hole[:2], oop_hole[2:]]

        self.dead_raws = set(oop_hole)
        ip_range = ranges[1]
        ip_index_candidates = list(range(self.total_holes[1]))
        shuffle(ip_index_candidates)

        # pylint: disable = undefined-loop-variable
        for ip_index in ip_index_candidates:
            ip_hole = ip_range[ip_index]
            if not any(card in ip_hole for card in self.dead_raws):
                break
        ip_hole = [ip_hole[:2], ip_hole[2:]]
        self.dead_raws.update(ip_hole)

        self.holes = [
            CardSet.from_cards(oop_hole),
            CardSet.from_cards(ip_hole),
        ]
        self.holes_index = [oop_index, ip_index]

    def play(self):
        if self.node is None:
            self.end()
            return

        player_id, children, *rest = self.node

        if player_id >= self.CHANCE_ID:
            candidates = list(children)
            shuffle(candidates)
            for candidate in candidates:
                if candidate in self.dead_raws:
                    continue
                self.node = children[candidate]
                self.deal(candidate)
                self.play()
                return

        strategy, evs = self.crop_tables(player_id, rest)
        actions = list(children)

        if player_id == self.human_id:
            self.refresh()

            table = [
                ["Action", "Weight", "EV"],
                *zip(actions, strategy, evs)
            ]
            self.info = [
                ' '.join(['Last decision point:', *(self.history or ['-'])]),
                tabulate(table, headers='firstrow', tablefmt='fancy_grid'),
            ]

            action = self.ask_action(actions)
        else:
            rng = random()
            for action, limit in zip(actions, strategy):
                if rng < limit:
                    break
                rng -= limit

        self.history.append(action)
        self.node = children[action]
        self.play()

    def crop_tables(self, player_id, tables):
        hole_index = self.holes_index[player_id]
        return [
            [
                round(value, 3)
                for value in table[hole_index::self.total_holes[player_id]]
            ]
            for table in tables
        ]

    def deal(self, raw):
        """Add a card to the board."""
        card = Card.from_raw(raw)
        self.board.append(card)
        self.history.append(f'/ {card} /')
        self.dead_raws.add(raw)

    def refresh(self):
        if os.name == 'posix':
            os.system('printf "\033c"')
        else:
            os.system('cls')

        self.log('Hole:', self.holes[self.human_id])

        for line in self.info:
            self.log(line)

        self.log()
        self.log('Board:', self.board)
        self.log(' '.join(self.history))
        self.log()

    def end(self):
        for line in self.info:
            self.log(line)

        self.log(' '.join(self.history))
        self.log('Vilain cards', self.holes[1 - self.human_id])
        self.log()
        _ = input('--- go to next hand ---')
