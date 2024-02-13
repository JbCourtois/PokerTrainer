from glob import glob

DEFAULT_FILE = './tests/fixtures/toy_game_AKK7.txt'
FILES = glob('Spots/**/*.txt', recursive=True) or [DEFAULT_FILE]
