import json

from .hand import Hand


class Game:
    def __init__(self):
        self.pot = None
        self.stack = None
        self.board = None
        self.tree = []
        self.ranges = []

    @staticmethod
    def from_file(filename):
        return TreeParser(filename).game

    def run(self):
        while True:
            hand = Hand(self)
            hand.play()


class TreeParser:
    def __init__(self, file):
        self.game = Game()
        self.stream = None
        self.line = None

        with open(file, encoding='utf8') as self.stream:
            self.parse_headers()
            self.game.tree = self.parse_node()

    def parse_headers(self):
        self.game.pot, self.game.stack = self._read()
        self.game.board = self._read().split(',')
        self.game.ranges = [self._read() for _ in range(2)]

    def parse_node(self, level=0):
        if not self._peek().startswith('\t' * level):
            return None

        player_id = self._consume()
        children = {}
        node = [player_id, children]

        if player_id <= 1:
            # Weights + EVs
            node.extend(self._read() for _ in range(2))

        while (child := self._peek()).startswith('\t' * level):
            if not child:
                break  # End of file

            child = self._consume()
            children[child] = self.parse_node(level=level + 1)

        return node

    def _peek(self):
        if self.line is None:
            self.line = self.stream.readline()
        return self.line

    def _consume(self):
        line = self._peek()
        self.line = None
        return json.loads(line)

    def _read(self):
        return json.loads(self.stream.readline())
