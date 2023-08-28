import os
from glob import glob

DEFAULT_FILE = './tests/fixtures/toy_game_AKK7.txt'
FILES = [
	file for file in glob('Spots/**/*.txt', recursive=True)
	if 'local' not in file.split(os.sep)
] or [DEFAULT_FILE]
