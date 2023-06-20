from trainer.walker import FILES
from trainer.game import Game


if __name__ == '__main__':
    print('Trainings:')
    for index, path in enumerate(FILES):
        print(index, '-', path)

    while True:
        try:
            index = int(input('Your choice? '))
            path = FILES[index]
            break
        except Exception:
            pass

    game = Game.from_file(path)
    game.run()
