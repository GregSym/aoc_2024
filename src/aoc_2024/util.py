import datetime
import re
import requests
import configparser


class DayHomepage:
    def __init__(self, day: int = 1, year: int | None = None) -> None:
        key = configparser.ConfigParser()
        key.read(".env")
        self.key = key.get("API", "session")
        self.day = day
        self.year = year if year is not None else datetime.datetime.now().year
        self.year_url = f"https://adventofcode.com/{self.year}/day"

    @property
    def url(self) -> str:
        return f"{self.year_url}/{self.day}"

    def get_day(self) -> str:
        res = requests.get(self.url, cookies={"session": self.key})
        if res.status_code == 200:
            return res.text
        else:
            return f"BAD_RESPONSE_{res.status_code}"

    def past_answers(self) -> list[int | str]:
        body = self.get_day()
        print(body)
        pattern = re.compile(r"Your puzzle answer was \<code\>(?P<answer>[^\<]*?)\<")
        return [
            int(m["answer"]) if m["answer"].isdigit() else m["answer"]
            for m in pattern.finditer(body)
        ]


if __name__ == "__main__":
    print(DayHomepage(4).past_answers())
