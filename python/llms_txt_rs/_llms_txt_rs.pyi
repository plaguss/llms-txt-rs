from typing import TypedDict, Optional

class LinkItem(TypedDict):
    title: str
    url: str
    desc: Optional[str]

class ParsedStructure(TypedDict):
    title: str
    summary: Optional[str]
    info: Optional[str]
    sections: dict[str, list[LinkItem]]

def parse_llms_txt(txt: str) -> ParsedStructure:
    """Parse /llms.txt files, go to https://llmstxt.org/ for more info.

    Args:
        txt (str): The content of the llms.txt file.

    Returns:
        ParsedStructure: The parsed file structure.
    """
    ...
