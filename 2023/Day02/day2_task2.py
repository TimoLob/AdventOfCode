class Game:
    def __init__(self) -> None:
        self.cubes = {"blue": 0, "red": 0, "green": 0}

    def power(self, description: str):
        cubes = {"blue": 0, "red": 0, "green": 0}
        for pull in description.strip().split(";"):
            for entry in pull.strip().split(","):
                count, color = entry.strip().split(" ")
                if cubes[color] < int(count):
                    cubes[color] = int(count)

        return cubes["blue"] * cubes["red"] * cubes["green"]


if __name__ == "__main__":
    total = 0
    with open("input.txt") as infile:
        lines = infile.readlines()
        for line in lines:
            game_id, description = line.strip().split(":")

            g = Game()
            total += g.power(description)
    print(total)
