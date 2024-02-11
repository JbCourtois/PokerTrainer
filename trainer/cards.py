from collections import UserList, UserString
from typing import NamedTuple
from dataclasses import dataclass
from random import shuffle

from termcolor import colored

import colorama
colorama.init()


class IndexableMixin:
    _auto_id = 0

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.__index = IndexableMixin._auto_id
        IndexableMixin._auto_id += 1

    def __lt__(self, other):
        # pylint: disable = protected-access
        return self.__index < other.__index

    def __gt__(self, other):
        # pylint: disable = protected-access
        return self.__index > other.__index


class Rank(IndexableMixin, UserString):
    pass


RANKS = [Rank(rank) for rank in [
    '2', '3', '4', '5', '6', '7', '8', '9',
    'T', 'J', 'Q', 'K', 'A',
]]
RANKS_MAP = {str(rank): rank for rank in RANKS}


@dataclass
class Suit(IndexableMixin):
    letter: str
    symbol: str
    color: str

    def __str__(self):
        return self.symbol


SUITS = [
    Suit('c', '♣', 'green'),
    Suit('d', '♦', 'yellow'),
    Suit('h', '♥', 'red'),
    Suit('s', '♠', 'white'),
]
SUITS_MAP = {
    suit.letter: suit
    for suit in SUITS
}


class Card(NamedTuple):
    rank: Rank
    suit: Suit

    def __str__(self):
        text = ''.join(map(str, [self.rank, self.suit]))
        return colored(text, self.suit.color, attrs=['bold'])

    def __lt__(self, other):
        return (self.rank, self.suit) < (other.rank, other.suit)

    def __gt__(self, other):
        return (self.rank, self.suit) > (other.rank, other.suit)

    def raw(self):
        return ''.join(map(str, [self.rank, self.suit.letter]))

    @classmethod
    def from_raw(cls, raw):
        rank = RANKS_MAP[raw[0]]
        suit = SUITS_MAP[raw[1]]
        return cls(rank, suit)


class CardSet(UserList):
    def __str__(self):
        return ' '.join(map(str, self))

    def raw(self):
        return ''.join(map(Card.raw, self))

    def bulkpop(self, nb_cards):
        return self.__class__(self.pop() for _ in range(nb_cards))

    @classmethod
    def from_cards(cls, cards):
        return cls(Card.from_raw(str(card)) for card in cards)


def generate_deck():
    deck = CardSet(Card(rank, suit) for rank in RANKS for suit in SUITS)
    shuffle(deck)
    return deck
