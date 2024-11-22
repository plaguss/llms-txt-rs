from textwrap import dedent

from llms_txt_rs import parse_llms_txt


def test_base_parse():
    llms_txt = dedent(
        """\
        # Title

        > Optional description goes here

        Optional details go here

        ## Section name

        - [Link title](https://link_url): Optional link details

        ## Optional

        - [Link title](https://link_url)
        """
    )
    data = parse_llms_txt(llms_txt)

    assert data == {
        "title": "Title",
        "summary": "Optional description goes here",
        "info": "Optional details go here",
        "sections": {
            "Section name": [
                {
                    "title": "Link title",
                    "url": "https://link_url",
                    "desc": "Optional link details",
                }
            ],
            "Optional": [
                {"title": "Link title", "url": "https://link_url", "desc": None}
            ],
        },
    }


def test_multiple_links():
    llms_txt = dedent(
        """\
        # Another Title

        > Another description

        More details

        ## Links

        - [Google](https://google.com): Search engine
        - [OpenAI](https://openai.com)

        ## More Info

        - [Python](https://python.org): Programming language
        """
    )
    data = parse_llms_txt(llms_txt)

    assert data == {
        "title": "Another Title",
        "summary": "Another description",
        "info": "More details",
        "sections": {
            "Links": [
                {
                    "title": "Google",
                    "url": "https://google.com",
                    "desc": "Search engine",
                },
                {"title": "OpenAI", "url": "https://openai.com", "desc": None},
            ],
            "More Info": [
                {
                    "title": "Python",
                    "url": "https://python.org",
                    "desc": "Programming language",
                }
            ],
        },
    }


def test_missing_optional_links():
    llms_txt = dedent(
        """\
        # Only Title

        Only details here

        ## Section

        - [Item](https://example.com)
        """
    )
    data = parse_llms_txt(llms_txt)

    assert data == {
        "title": "Only Title",
        "summary": None,
        "info": "Only details here",
        "sections": {
            "Section": [{"title": "Item", "url": "https://example.com", "desc": None}]
        },
    }


def test_no_links():
    llms_txt = dedent(
        """\
        # No Links Title
    
        > No description

        Some details without links
        """
    )
    data = parse_llms_txt(llms_txt)

    assert data == {
        "title": "No Links Title",
        "summary": "No description",
        "info": "Some details without links",
        "sections": {},
    }
