from unittest import TestCase
from unittest.mock import patch

from trainer.cards import generate_deck


def mock_color(text, *args, **kwargs):
    return text


@patch('trainer.cards.colored', mock_color)
class TestDeck(TestCase):
    def test_deck(self):
        deck = generate_deck()
        deck.sort()

        firsts = deck[:6]
        self.assertEqual(str(firsts), '2♣ 2♦ 2♥ 2♠ 3♣ 3♦')
        self.assertEqual(firsts.raw(), '2c2d2h2s3c3d')

        lasts = deck.bulkpop(6)
        self.assertEqual(str(lasts), 'A♠ A♥ A♦ A♣ K♠ K♥')
        self.assertEqual(lasts.raw(), 'AsAhAdAcKsKh')
