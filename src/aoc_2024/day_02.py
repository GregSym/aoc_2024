from aoc_2022.utils.day_handler import DayInterface
from aoc_2022.utils.transforms import DataTransforms
from aoc_2024.aoc_2024 import solve_day_02_pt_01
import pytest

test_input = """7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"""


@pytest.fixture
def input():
    return test_input


def solve_day(input: str) -> int:
    info = DataTransforms(input).lines  # manipulate input per usecase
    # solve
    return solve_day_02_pt_01(input)


def test_day_2_part_1(input: str) -> None:
    # test solution to part 1
    assert 2 == solve_day(input)


# def test_day_1_part_2(input: str) -> None:
#    # test solution to part 2
#    assert 19 == solve_day(input)


if __name__ == "__main__":
    real_input = DayInterface(2).get_day()
    test_day_2_part_1(test_input)
    # test_day_1_part_2(test_input)
    print(DayInterface(2).submit_day(solve_day(real_input)))
