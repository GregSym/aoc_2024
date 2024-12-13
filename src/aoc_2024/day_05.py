from aoc_2022.utils.day_handler import DayInterface
from aoc_2022.utils.transforms import DataTransforms
import pytest

test_input = """mjqjpqmgbljsphdztnvjfqwrcgsmlb"""


@pytest.fixture
def input():
    return test_input


def solve_day(input: str) -> int:
    info = DataTransforms(input).lines  # manipulate input per usecase
    # solve
    return 0


def test_day_5_part_1(input: str) -> None:
    # test solution to part 1
    assert 7 == solve_day(input)


# def test_day_5_part_2(input: str) -> None:
#    # test solution to part 2
#    assert 19 == solve_day(input)


if __name__ == "__main__":
    real_input = DayInterface(5).get_day()
    test_day_5_part_1(test_input)
    # test_day_5_part_2(test_input)
    print(DayInterface(5).submit_day(solve_day(real_input)))
