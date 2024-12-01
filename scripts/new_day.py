import argparse

parser = argparse.ArgumentParser(description='Create new day file.')
parser.add_argument('--year', type=int, help='year',
                    default=2023, choices=[2019, 2021, 2022, 2023, 2024])
parser.add_argument('--day', type=int, help='day', required=True)

args = parser.parse_args()

with open("./scripts/day_template.rs") as f:
    template = f.read()

new_content = template.replace(
    "Day1", f"Day{args.day}").replace("2019", str(args.year))

target_path = f"./src/aoc{args.year}/day{args.day}.rs"
with open(target_path, "w") as f:
    f.write(new_content)
