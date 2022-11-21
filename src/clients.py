from abc import ABCMeta, abstractmethod
from random import random

from .cards import Card, CardSet


class Client(metaclass=ABCMeta):
    name = 'BaseClient'

    def __init__(self, cards):
        pass

    @abstractmethod
    def get_action(self, strategy):
        pass


class HumanClient(Client):
    name = 'Human'

    def __init__(self, cards):
        cards = CardSet(Card.from_raw(str(raw)) for raw in cards)
        print('--- New hand ---')
        print('Your cards:', cards)

    def get_action(self, strategy):
        print()
        print('Choices:')
        for index, action in enumerate(strategy['actions']):
            print(f'{index}: {action}')
        print()

        while True:
            try:
                index = int(input('Your action? '))
                return strategy['actions'][index]
            except (ValueError, IndexError):
                pass

class BotClient(Client):
    name = 'Bot'

    def __init__(self, cards):
        self.cards = ''.join(str(raw) for raw in cards)

    def get_action(self, strategy):
        roll = random()
        for index, weight in enumerate(strategy['strategy'][self.cards]):
            if roll <= weight:
                break
            roll -= weight

        action = strategy['actions'][index]
        print()
        print('Vilain:', action)

        return strategy['actions'][index]
