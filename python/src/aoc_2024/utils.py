from pathlib import Path
from typing import Iterator, Protocol


class FromLines(Protocol):
    @classmethod
    def from_lines(cls, lines: Iterator[str]) -> "FromLines": ...

    @classmethod
    def read(cls, filename: str) -> "FromLines":
        path = Path(__file__).parent.parent.parent.parent / "data" / filename
        if not path.exists():
            raise FileNotFoundError(f"File not found: {path}")

        with open(path) as f:
            lines = (line.strip() for line in f)
            return cls.from_lines(lines)
