import os
from io import StringIO
from random import seed

from unittest import TestCase
from unittest.mock import patch

from src.game import Game
from src.hand import Hand

base_path = os.path.dirname(__file__)


class TestGame(TestCase):
    @classmethod
    def setUpClass(cls):
        path = base_path + '/fixtures/sample_hand'
        seed(40)

        cls.game = Game(path + '/output_parameters.txt', path + '/output_strategy.json')
        cls.hand = Hand(cls.game)

    @patch('src.clients.input')
    def test_hand(self, mock_input):
        mock_input.side_effect = [0] * 1000
        self.hand.play()
        # iii = StringIO()

        # print(self.hand.board, file=iii)
        # print(self.hand.board)
        # self.assertEqual(iii.getvalue(), '7♣ J♥ 2♥ Q♠')
