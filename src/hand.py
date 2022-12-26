from random import random, choice

from .cards import Card, CardSet


class LoggerMixin:
    def __init__(self, *args, logger=None, **kwargs):
        super().__init__(*args, **kwargs)
        self.logger = logger

    def log(self, *args, **kwargs):
        print(*args, **kwargs, file=self.logger)


class Hand(LoggerMixin):
    def __init__(self, game):
        super().__init__()
        self.__dict__.update(game.__dict__)

        dead_raws = self.board.split(',')
        self.board = CardSet(Card.from_raw(raw) for raw in dead_raws)
        self.dead_raws = set(dead_raws)

        self._init_cards()
        self.is_over = False
        self.clients = [client(cards) for client, cards in zip(self.clients, self.cards)]

    def _init_cards(self):
        self.cards = [None, None]
        for _ in range(10):
            if self._try_cards():
                return

        raise ValueError('Could not find a valid deal')

    def _try_cards(self):
        new_dead_raws = set(self.dead_raws)
        for player, rrr in enumerate(self.ranges):
            cards, weight = choice(list(rrr.items()))
            raws = [str(card) for card in cards]
            if (not new_dead_raws.isdisjoint(raws)) or random() > weight:
                return False
            self.cards[player] = cards
            new_dead_raws.update(raws)

        self.dead_raws = new_dead_raws
        return True

    def play(self):
        while not self.is_over:
            self.act()
        self.end()

    def act(self):
        if self.tree['node_type'] == 'action_node':
            client = self.clients[self.tree['player']]
            action = client.get_action(self.tree['strategy'])

            if (childrens := self.tree.get('childrens')) is None:
                self.is_over = True
                return

            self.tree = childrens.get(action)
            if self.tree is None:
                self.is_over = True
                return

        elif self.tree['node_type'] == 'chance_node':
            next_cards = self.tree.get('dealcards')
            if next_cards is None:
                self.is_over = True
                return

            card_choices = [card for card in next_cards if card not in self.dead_raws]
            next_card = choice(card_choices)
            self.dead_raws.add(next_card)
            self.tree = next_cards[next_card]
            self.board.append(Card.from_raw(next_card))
            print('Board:', self.board)
            return

    def end(self):
        print()
        print('OOP:', CardSet.from_cards(self.cards[1]))
        print(' IP:', CardSet.from_cards(self.cards[0]))
