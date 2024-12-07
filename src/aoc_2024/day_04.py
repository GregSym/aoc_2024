from typing import Generator, Self
from aoc_2022.utils.day_handler import DayInterface
from aoc_2022.utils.transforms import DataTransforms
from aoc_2024.aoc_2024 import solve_day_04_pt_01
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


@pytest.fixture
def input():
    return test_input


TARGET = "XMAS"


class Mask(list[list[str]]):
    @classmethod
    def from_input(cls, input: list[str]) -> Generator[Self, None, None]:
        for rows in zip(*[input[i:] for i in range(len(TARGET))]):
            for i, c in enumerate(rows[0]):

                def grab_horizontal_splits(row: str) -> list[str]:
                    return [c for c in row[i : min(i + len(TARGET), len(row))]]

                if len(grab_horizontal_splits(rows[0])) == len(TARGET):
                    yield cls([grab_horizontal_splits(row) for row in rows])

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

    @property
    def words(self) -> list[str]:
        return ["".join([self[j][i] for (i, j) in path]) for path in self.indeces]

    @property
    def eval(self) -> int:
        return sum([word == TARGET for word in self.words])


class Grid(list[Mask]):
    @classmethod
    def from_input(cls, input: list[str]) -> Self:
        return cls([mask for mask in Mask.from_input(input=input)])

    @property
    def xmas_count(self) -> int:
        return sum([mask.eval for mask in self])


def solve_day(input: str) -> int:
    info = DataTransforms(input).lines  # manipulate input per usecase
    print(Grid.from_input(info).xmas_count)
    # solve
    return solve_day_04_pt_01(input)


def test_day_4_part_1(input: str) -> None:
    # test solution to part 1
    result = solve_day(input)
    assert (
        18 == result
    ), f"{result=} alt solution {Grid.from_input(DataTransforms(input).lines).xmas_count}"


# def test_day_4_part_2(input: str) -> None:
#    # test solution to part 2
#    assert 19 == solve_day(input)


if __name__ == "__main__":
    real_input = DayInterface(4).get_day()
    test_day_4_part_1(test_input)
    # test_day_4_part_2(test_input)
    print(DayInterface(4).submit_day(solve_day(real_input)))
