import requests
import os
import datetime
from dotenv import load_dotenv

load_dotenv()

token = os.environ["AOC_SESSION"]
cookies = {"session": token}

for year in [2019, 2021, 2022, 2023]:
    for day in range(1, 25 + 1):
        dt = datetime.datetime(year, 12, day)
        path = f"inputs/{year}/day{day}.txt"
        if dt < datetime.datetime.now() and not os.path.exists(path):
            print(f"Fetching input for {dt}")
            response = requests.get(
                f"https://adventofcode.com/{year}/day/{day}/input", cookies=cookies
            )
            with open(path, "w") as f:
                f.write(response.text)
