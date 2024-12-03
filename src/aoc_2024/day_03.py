from aoc_2022.utils.day_handler import DayInterface
from aoc_2022.utils.transforms import DataTransforms
from aoc_2024.aoc_2024 import solve_day_03_pt_01
import pytest

test_input = """xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"""


@pytest.fixture
def input():
    return test_input


def solve_day(input: str) -> int:
    info = DataTransforms(input).lines  # manipulate input per usecase
    # solve
    return solve_day_03_pt_01(input)


def test_day_3_part_1(input: str) -> None:
    # test solution to part 1
    assert 161 == solve_day(input), f"{solve_day(input)=}"


# def test_day_3_part_2(input: str) -> None:
#    # test solution to part 2
#    assert 19 == solve_day(input)


if __name__ == "__main__":
    real_input = DayInterface(3).get_day()
    test_day_3_part_1(test_input)
    # test_day_3_part_2(test_input)
    print(DayInterface(3).submit_day(solve_day(real_input)))
