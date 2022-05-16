from ..quality import QUALITIES
from ..formats import get_available_formats


def list_qualities_and_formats():
    print('Available qualities:')
    for ext, info in QUALITIES.items():
        print(f"Qualities for {ext}:")
        for key, val in info.items():
            print(f"{key:2}: {val.name}")
    print()
    formats = get_available_formats()
    print("Available formats:")
    for format_ in formats:
        print(f'- {format_}')
