import collections
from typing import Generator, Self
from aoc_2022.utils.day_handler import DayInterface
from aoc_2022.utils.transforms import DataTransforms
from aoc_2024.aoc_2024 import solve_day_04_pt_01, solve_day_04_pt_02
import pytest

test_input = """MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"""

test_alt = """..X...
.SAMX.
.A..A.
XMAS.S
.X...."""


@pytest.fixture
def input():
    return test_input


@pytest.fixture
def input_alt():
    return test_alt


TARGET = "XMAS"


class Grid(collections.defaultdict[tuple[int, int], str]):
    @classmethod
    def from_input(cls, text: list[str]) -> Self:
        default_dict = cls(lambda: "")
        default_dict.update(
            {(x, y): c for y, row in enumerate(text) for x, c in enumerate(row)}
        )
        return default_dict

    @property
    def indeces(self) -> Generator[list[tuple[int, int]], None, None]:
        inc = [i for i in range(len(TARGET))]
        dec = [i for i in range(len(TARGET) - 1, -1, -1)]
        for row in range(len(TARGET)):
            yield [(i, j) for (i, j) in zip([row] * len(TARGET), inc)]
            yield [(i, j) for (i, j) in zip([row] * len(TARGET), dec)]
        for col in range(len(TARGET)):
            yield [(i, j) for (i, j) in zip(inc, [col] * len(TARGET))]
            yield [(i, j) for (i, j) in zip(dec, [col] * len(TARGET))]
        yield [(i, j) for (i, j) in zip(inc, inc)]
        yield [(i, j) for (i, j) in zip(dec, dec)]
        yield [(i, j) for (i, j) in zip(inc, dec)]
        yield [(i, j) for (i, j) in zip(dec, inc)]

    def coords(self, x: int, y: int) -> Generator[list[tuple[int, int]], None, None]:
        for path in self.indeces:
            yield [(x + x_offset, y + y_offset) for x_offset, y_offset in path]

    def words_at(
        self, x: int, y: int
    ) -> Generator[tuple[frozenset[tuple[int, int]], str]]:
        for path in self.coords(x=x, y=y):
            yield frozenset(path), "".join([self.get((x, y), "") for x, y in path])

    @property
    def target_words(self) -> set[tuple[frozenset[tuple[int, int]], str]]:
        return {
            (path, word)
            for k in self
            for path, word in self.words_at(*k)
            if word == TARGET
        }

    @property
    def xmas_count(self) -> int:
        return len(self.target_words)


def solve_day(input: str) -> int:
    info = DataTransforms(input).lines  # manipulate input per usecase
    print(Grid.from_input(info).xmas_count)
    # solve
    return solve_day_04_pt_01(input)


def solve_day_2(input: str) -> int:
    info = DataTransforms(input).lines  # manipulate input per usecase
    # print(Grid.from_input(info).xmas_count)
    # solve
    return solve_day_04_pt_02(input)


def test_day_4_part_1(input: str) -> None:
    # test solution to part 1
    result = solve_day(input)
    assert (
        18 == result
    ), f"{result=} alt solution {Grid.from_input(DataTransforms(input).lines).xmas_count}"


def test_day_4_part_1_alt(input_alt: str) -> None:
    # test solution to part 1
    result = solve_day(input_alt)
    assert (
        4 == result
    ), f"{result=} alt solution {Grid.from_input(DataTransforms(input_alt).lines).xmas_count}"


def test_day_4_part_2(input: str) -> None:
    # test solution to part 2
    solution = solve_day_2(input)
    assert 9 == solution, f"{solution=}"


if __name__ == "__main__":
    real_input = DayInterface(4).get_day()
    test_day_4_part_1(test_input)
    test_day_4_part_1_alt(test_alt)
    test_day_4_part_2(test_input)
    print(DayInterface(4).submit_day(solve_day(real_input)))
    print(DayInterface(4).submit_day(solve_day_2(real_input), part=2))
