from collections import deque
import json
from random import randrange

import eval7

# from .hand import HandUI
from .clients import HumanClient, BotClient


def parse_range(raw):
    hrange = eval7.HandRange(','.join(
        parse_fragment(fragment) for fragment in raw.split(',')
    ))
    return dict(hrange.hands)


def parse_fragment(fragment):
    token, weight = fragment.split(':')
    return f'{weight}({token})'


class Game:
    def __init__(self, parameters, strategy):
        self.clients = deque([HumanClient, BotClient])
        self.clients.rotate(randrange(2))
        self.ranges = [None, None]

        with open(parameters) as file:
            self.parse_parameters(file)

        with open(strategy) as file:
            self.tree = json.load(file)

    def parse_parameters(self, stream):
        while line := stream.readline():
            self.parse_parameter(line.strip())

    def parse_parameter(self, line):
        header, *rest = line.split()

        if header == 'set_pot':
            self.pot = int(rest[0])
        elif header == 'set_effective_stack':
            self.stack = int(rest[0])
        elif header == 'set_board':
            self.board = rest[0]
        elif header == 'set_range_oop':
            self.ranges[1] = parse_range(rest[0])
        elif header == 'set_range_ip':
            self.ranges[0] = parse_range(rest[0])
