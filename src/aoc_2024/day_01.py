from aoc_2022.utils.day_handler import DayInterface
from aoc_2022.utils.transforms import DataTransforms
import pytest

test_input = """3   4
4   3
2   5
1   3
3   9
3   3"""


@pytest.fixture
def input():
    return test_input


def solve_day(input: str) -> int:
    info = DataTransforms(input).lines  # manipulate input per usecase
    parsed_left = []
    parsed_right = []
    for row in info:
        l, r = row.split()
        l_parsed, r_parsed = int(l), int(r)
        parsed_left.append(l_parsed)
        parsed_right.append(r_parsed)
    parsed_left.sort()
    parsed_right.sort()
    return sum([abs(l - r) for l, r in zip(parsed_left, parsed_right)])


def test_day_1_part_1(input: str) -> None:
    # test solution to part 1
    assert 11 == solve_day(input)


# def test_day_1_part_2(input: str) -> None:
#    # test solution to part 2
#    assert 19 == solve_day(input)


if __name__ == "__main__":
    real_input = DayInterface(1).get_day()
    test_day_1_part_1(test_input)
    # test_day_1_part_2(test_input)
    print(DayInterface(1).submit_day(solve_day(real_input)))
