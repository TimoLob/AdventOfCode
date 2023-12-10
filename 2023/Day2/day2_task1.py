class Game:
    def __init__(self) -> None:
        self.cubes = {"blue": 0, "red": 0, "green": 0}

    def possible(self, description: str, red=12, green=13, blue=14):
        for pull in description.strip().split(";"):
            cubes = {"blue": 0, "red": 0, "green": 0}
            for entry in pull.strip().split(","):
                count, color = entry.strip().split(" ")
                cubes[color] += int(count)
                if not (
                    cubes["red"] <= red
                    and cubes["green"] <= green
                    and cubes["blue"] <= blue
                ):
                    return False
        return True


if __name__ == "__main__":
    total = 0
    with open("input.txt") as infile:
        lines = infile.readlines()
        for line in lines:
            game_id, description = line.strip().split(":")

            g = Game()
            if g.possible(description):
                game_id = int(game_id.split(" ")[-1])
                total += game_id
    print(total)
