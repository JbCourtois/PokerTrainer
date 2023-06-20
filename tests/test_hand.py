import os
from io import StringIO
from random import seed

from unittest import TestCase
from unittest.mock import patch

from trainer.game import Game
from trainer.hand import Hand
from trainer.walker import DEFAULT_FILE

base_path = os.path.dirname(__file__)


class TestGame(TestCase):
    @patch('os.system')
    @patch('trainer.hand.input')
    def test_hand(self, mock_input, *mocks):
        seed(40)
        mock_input.side_effect = [0] * 1000

        logger = StringIO()
        game = Game.from_file(base_path + '/fixtures/toy_game_AKK7.txt')
        hand = Hand(game, logger=logger)
        hand.play()

        self.assertEqual(hand.dead_raws, {'Ad', 'Jd', '7s', '3h', '2h'})
        self.assertEqual(hand.holes_index, [1, 1])

        with open(base_path + '/fixtures/expected_log.log') as file:
            self.assertEqual(logger.getvalue(), file.read())

    def test_walker_default(self):
        self.assertTrue(
            os.path.exists(DEFAULT_FILE),
            f"Default trainer file {DEFAULT_FILE} does not exist"
        )
