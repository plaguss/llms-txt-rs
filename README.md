# llms-txt-rs
Parser for [llms.txt](https://llmstxt.org/) files, written in rust just for the sake of it, with bindings in python.

## Installation

**llms_txt_rs** requires Python 3.8 - 3.13.

```bash
uv pip install llms_txt_rs
```

Binaries are available for:

- Linux: x86_64, aarch64, i686, armv7l, musl-x86_64 & musl-aarch64
- MacOS: x86_64 & arm64
- Windows: amd64 & win32

Otherwise, you can install from source which requires Rust stable to be installed.

## How it works

The API consists of a single function:

```python
import requests
from llms_txt_rs import parse_llms_txt

txt = requests.get("https://llmstxt.org/llms.txt").text
parsed = parse_llms_txt(txt)
```

Which would yield something like the following:

```json
{
  "title": "llms.txt",
  "summary": null,
  "info": "> A proposal that those interested in providing LLM-friendly content add a /llms.txt file to their site. This is a markdown file that provides brief background information and guidance, along with links to markdown files providing more detailed information.",
  "sections": {
    "Docs": [
      {
        "title": "llms.txt proposal",
        "url": "https://llmstxt.org/index.md",
        "desc": "The proposal for llms.txt"
      },
      {
        "title": "Python library docs",
        "url": "https://llmstxt.org/intro.html.md",
        "desc": "Docs for `llms-txt` python lib"
      },
      {
        "title": "ed demo",
        "url": "https://llmstxt.org/ed-commonmark.md",
        "desc": "Tongue-in-cheek example of how llms.txt could be used in the classic `ed` editor, used to show how editors could incorporate llms.txt in general."
      }
    ]
  }
}
```

## Releasing

WIP
