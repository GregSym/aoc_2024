from aoc_2022.utils.day_handler import DayInterface
from aoc_2022.utils.transforms import DataTransforms
from aoc_2024.aoc_2024 import solve_day_05_pt_01
import pytest

test_input = """47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"""


@pytest.fixture
def input():
    return test_input


def solve_day(input: str) -> int:
    info = DataTransforms(input).lines  # manipulate input per usecase
    # solve
    return solve_day_05_pt_01(input)


def test_day_5_part_1(input: str) -> None:
    # test solution to part 1
    assert 143 == solve_day(input), f"{solve_day(input)=}"


# def test_day_5_part_2(input: str) -> None:
#    # test solution to part 2
#    assert 19 == solve_day(input)


if __name__ == "__main__":
    real_input = DayInterface(5).get_day()
    test_day_5_part_1(test_input)
    # test_day_5_part_2(test_input)
    print(DayInterface(5).submit_day(solve_day(real_input)))
