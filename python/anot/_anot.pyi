from pathlib import Path
from typing import List, Optional

__all__ = [
    "Annotation",
    "Location",
    "SyntaxContext",
    "extract_annotations",
    "format_annotations",
    "run_cli",
]

class SyntaxContext:
    node_type: str
    parent_type: str
    associated_name: Optional[str]
    variable_name: Optional[str]
    def __init__(
        self,
        *,
        node_type: str,
        parent_type: str,
        associated_name: Optional[str],
        variable_name: Optional[str],
    ) -> None: ...

class Location:
    file: Path
    line: int
    inline: bool
    def __init__(self, *, file: Path, line: int, inline: bool) -> None: ...

class Annotation:
    kind: str
    content: str
    location: Location
    contest: SyntaxContext
    def __init__(self, *, kind: str, content: str, location: Location) -> None: ...

def extract_annotations(content: str, file_type: str) -> List[Annotation]: ...
def format_annotations(annotations: List[Annotation], format: str) -> str: ...
def run_cli(args: list[str]): ...
