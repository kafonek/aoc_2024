from dataclasses import dataclass
from typing import Iterator

from aoc_2024.utils import FromLines


@dataclass
class Day02(FromLines):
    reports: list[list[int]]

    @classmethod
    def from_lines(cls, lines: Iterator[str]) -> "Day02":
        reports = []
        for line in lines:
            nums = [int(n) for n in line.split() if n.isdigit()]
            reports.append(nums)
        return cls(reports=reports)

    def is_safe_report(self, report: list[int]) -> bool:
        if len(report) < 2:
            return False

        # Check first pair to determine if we should be increasing or decreasing
        increasing = report[1] > report[0]
        prev = report[0]

        for curr in report[1:]:
            diff = curr - prev

            # Check rules:
            # 1. Must maintain increasing/decreasing pattern
            # 2. Difference must be between 1 and 3 (inclusive)
            if increasing and (diff <= 0 or diff > 3):
                return False
            if not increasing and (diff >= 0 or diff < -3):
                return False

            prev = curr

        return True

    def count_safe_reports(self) -> int:
        return sum(1 for report in self.reports if self.is_safe_report(report))

    def count_safe_reports_with_dampener(self) -> int:
        safe_count = 0
        for report in self.reports:
            # First check if it's safe without removing anything
            if self.is_safe_report(report):
                safe_count += 1
                continue

            # Try removing each number one at a time
            for skip_idx in range(len(report)):
                filtered = [x for i, x in enumerate(report) if i != skip_idx]
                if self.is_safe_report(filtered):
                    safe_count += 1
                    break

        return safe_count


def part1(filename: str) -> int:
    data = Day02.read(filename)
    return data.count_safe_reports()


def part2(filename: str) -> int:
    data = Day02.read(filename)
    return data.count_safe_reports_with_dampener()
