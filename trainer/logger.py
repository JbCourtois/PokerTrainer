class LoggerMixin:
    # pylint: disable = too-few-public-methods
    def __init__(self, *args, logger=None, **kwargs):
        super().__init__(*args, **kwargs)
        self.logger = logger

    def log(self, *args, **kwargs):
        print(*args, **kwargs, file=self.logger)

    def ask_action(self, actions, return_index=False):
        self.log()
        self.log('Choices:')
        for index, action in enumerate(actions):
            self.log(f'{index}: {action}')
        self.log()

        while True:
            try:
                index = int(input('Your action? '))
                return index if return_index else actions[index]
            except (ValueError, IndexError):
                pass
