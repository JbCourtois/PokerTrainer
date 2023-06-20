import subprocess


class RustIO:
    def __init__(self):
        # Start Rust program as a subprocess
        self.program = subprocess.Popen(
            ["cargo", "run", "--release"],
            cwd="rust",
            stdout=subprocess.PIPE,
            stdin=subprocess.PIPE,
            stderr=subprocess.PIPE)

    def send(self, msg):
        self.program.stdin.write(user_input.encode() + b"\n")
        self.program.stdin.flush()

        output = self.program.stdout.readline()
        return (
            output.decode().strip()
            if output else None)


rust = RustIO()


# Read output until Rust program is ready
while True:
    line = rust.program.stderr.readline().decode()
    if line:
        print(line.strip())
    if line.strip() == "Ready":
        break

while True:
    try:
        user_input = input("Enter an integer: ")
    except KeyboardInterrupt:
        print(rust.program.stderr.read())
        rust.program.kill()
        break
