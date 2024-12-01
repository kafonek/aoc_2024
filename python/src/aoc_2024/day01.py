from collections import defaultdict
from dataclasses import dataclass
from typing import Iterator

from aoc_2024.utils import FromLines


@dataclass
class Day01(FromLines):
    left: list[int]
    right: list[int]

    @classmethod
    def from_lines(cls, lines: Iterator[str]) -> "Day01":
        left = []
        right = []

        for line in lines:
            nums = [int(n) for n in line.split() if n.isdigit()]
            if len(nums) >= 2:
                left.append(nums[0])
                right.append(nums[1])

        return cls(left=left, right=right)

    def distance(self) -> int:
        left_sorted = sorted(self.left)
        right_sorted = sorted(self.right)

        total_distance = 0
        for l, r in zip(left_sorted, right_sorted):  # noqa: E741
            distance = abs(l - r)
            total_distance += distance

        return total_distance

    def similarity(self) -> int:
        right_counts = defaultdict(int)
        total = 0

        for r in self.right:
            right_counts[r] += 1

        for l in self.left:  # noqa: E741
            val = l * right_counts[l]
            total += val

        return total


def part1(filename: str) -> int:
    data = Day01.read(filename)
    return data.distance()


def part2(filename: str) -> int:
    data = Day01.read(filename)
    return data.similarity()
