from ..quality import QUALITIES
from ..formats import get_available_formats


def list_qualities_and_formats():
    print('Available qualities:')
    for ext, info in QUALITIES.items():
        print(f"Qualities for {ext}:")
        for k, v in info.items():
            print(f"{k:2}: {v.name}")
    print()
    formats = get_available_formats()
    print("Available formats:")
    for f in formats:
        print(f'- {f}')
