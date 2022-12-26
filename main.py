import os

from src.walker import FOLDERS
from src.game import Game


if __name__ == '__main__':
    print('Trainings:')
    for index, path in enumerate(FOLDERS):
        print(index, '-', path)

    while True:
        try:
            index = int(input('Your choice? '))
            path = FOLDERS[index]
            break
        except Exception:
            pass

    parameters = os.path.join(path, 'output_parameters.txt')
    strategy = os.path.join(path, 'output_strategy.json')

    game = Game(parameters, strategy)
    game.run()
