from ..quality import QUALITIES


def validate_quality(args, parser):
    if not hasattr(args, 'quality'):
        return
    if hasattr(args, 'format') and args.quality != 0:
        parser.error("You can't use --quality with --format.")
    for ext in {"mp3", "ogg", "wav", "webm"}:
        if getattr(args, ext) and args.quality not in QUALITIES[ext]:
            parser.error(f"Invalid quality {args.quality} for {ext}.")
