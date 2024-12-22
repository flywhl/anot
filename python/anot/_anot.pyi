from typing import List

__all__ = ["Annotation", "extract_annotations", "format_annotations"]

class Annotation:
    kind: str
    content: str
    def __init__(self, kind: str, content: str) -> None: ...

def extract_annotations(content: str, file_type: str) -> List[Annotation]: ...
def format_annotations(annotations: List[Annotation], format: str) -> str: ...
