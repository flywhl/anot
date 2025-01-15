import sys

from ._anot import (
    Annotation,
    Location,
    SyntaxContext,
    extract_annotations,
    format_annotations,
    run_cli,
)

__all__ = [
    "Annotation",
    "Location",
    "SyntaxContext",
    "extract_annotations",
    "format_annotations",
    "run_cli",
]


def main():
    sys.exit(run_cli(sys.argv))
