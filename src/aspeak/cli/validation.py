from ..quality import QUALITIES


def validate_quality(args, parser):
    if not hasattr(args, 'quality'):
        return
    if hasattr(args, 'format') and args.quality != 0:
        parser.error("You can't use --quality with --format.")
    for ext in ["mp3", "ogg", "wav", "webm"]:
        if getattr(args, ext) and args.quality not in QUALITIES[ext]:
            parser.error(f"Invalid quality {args.quality} for {ext}.")


def get_ineffective_args_for_listing(args):
    result = [option for option in
              ['pitch', 'rate', 'style', 'role', 'style_degree', 'quality', 'format', 'encoding', 'file', 'text',
               'ssml'] if hasattr(args, option)]
    if args.output_path is not None:
        result.append('output_path')
    return ', '.join(result)


def has_text_options(args):
    return any(hasattr(args, option) for option in ['pitch', 'rate', 'style', 'role', 'style_degree'])
