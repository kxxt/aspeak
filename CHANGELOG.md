# aspeak Changelog

# v6.0.0-rc.1

Changes after v6.0.0-beta.3:

- Rust crate: make all items visible in the root module (Flat is better than nested).
- GitHub branches: The main branch has been deleted. The default branch is now `v5` and it will change to `v6` when v6 is released.
- Python binding: Now type hints are provided. You will get better completion in your IDE.

# v6.0.0-beta.3

Changes after v6.0.0-beta.2:

- Improve an error message of the python binding
- Enable abi3 wheels so that we do not need to build for every python version.

# v6.0.0-beta.2

Changes after v6.0.0-beta.1:

- CLI: performance improvement: eliminate unnecessary memory copy
- Docs(crate): Add two more examples
- Docs(crate): Add more doc comments

# v6.0.0-beta.1

Changes after v6.0.0-alpha.3:

- Feature: Add two methods to `RestSynthesizer` that returns `Bytes` instead of `Vec<u8>`.
- Upgrade openssl dependency (Solves security alert #77)
- Add two examples for the rust crate
  - 01-synthesize-txt-files.rs: Synthesize speech from \*.txt files in a directory.
  - 02-rssl.rs: RSSL, Read-Synthesize-Speak-Loop (Something similar to a REPL). Read text from stdin line by line, synthesize speech and play it.
- Internal refactor

# v6.0.0-alpha.3

Changes after v6.0.0-alpha.2:

- Improve doc comments.
- Bump `strum` to 0.25 by @attila-lin
- crate: support TLS feature flags
- crate: add `synthesizers` feature that enables all synthesizers

# v6.0.0-alpha.2

## For CLI users

There are no breaking changes. But there are some differences.

### Changes

- Now the CLI uses the REST API instead of the WebSocket API by default.
  - You can use the `--mode websocket` flag to use the WebSocket API.
  - You can also set the `mode` field to `websocket` in the auth section in your profile to use the WebSocket API by default.

### Bug Fixes

- When the TTS API returns empty audio, aspeak no longer reports an cryptic "Unrecognized format" error.
  - It will now report a warning in this case: "Got empty audio buffer, nothing to play"
- Now the voice listing command no longer fails with this API endpoint: https://speech.platform.bing.com/consumer/speech/synthesize/readaloud/voices/list

## For Rust crate users

There are lots of breaking changes.

- Some fields of the `Voice` struct is now optional.
- We now uses an [modular approach for error handling](https://sabrinajewson.org/blog/errors) instead of using a big enum for all errors.
- Now there are two synthesizers: `RestSynthesizer` for the REST API and `WebSocketSynthesizer` for the WebSocket API.
- There is a `UnifiedSynthesizer` trait that provides a unified interface for both synthesizers.
- Some methods are renamed. For example, `Synthesizer::connect` is now `Synthesizer::connect_websocket`.
- Three new features of this crate:
  - `rest-synthesizer`: Enable the `RestSynthesizer` struct.
  - `websocket-synthesizer`: Enable the `WebSocketSynthesizer` struct.
  - `unified-synthesizer`: Enable the `UnifiedSynthesizer` trait.
  - The above three features are enabled by default.
  - Disabling `websocket-synthesizer` feature will save you a bunch of dependencies. (`aspeak.rlib` is ~0.8MB smaller in release mode)
- Other minor changes.

## For Python binding users

One breaking change:

- The `SpeechService` now automatically connect when it is constructed. The `connect` method is removed.
- It now uses the REST API by default.
- New keyword argument for the constructor of `SpeechService`:
  - mode: `rest` or `websocket`. Default is `rest`.

# v5.2.0

## CLI

You can now set the authentication secrets via the following environment variables:

- `ASPEAK_AUTH_KEY` for authentication using subscription key
- `ASPEAK_AUTH_TOKEN` for authentication using authorization token

## Rust API

- Now you can use `Voice::request_available_voices`(or `Voice::request_available_voices_with_additional_headers`) to get the list of available voices.

# v5.1.0

- Add binary feature to aspeak crate to make rust lib less bloated
  - From now on, building the CLI requires `-F binary` flag.

# v5.0.1-alpha.2

- Add binary feature to make rust lib less bloated

# v5.0.0

## Enhancements

- Add support for `--color={auto,always,never}` options. And `aspeak` will also respect the `NO_COLOR` environment variable.
  - There is an edge case that `aspeak` will use colored output even if `--color=never` is specified.
    This is because `aspeak` uses `clap` to parse command line options. `--color=never` works only if the command line parsing is successful.
    So if you specify an invalid option, `aspeak` will print the error message and exit. In this case, `aspeak` will use colored output.
- More documentation for the rust crate.
- Minor performance improvements.
- Now you can specify the custom voice list API url in your profile(field `voice_list_api` in section `auth`).

## Breaking changes

- The default trial endpoint has been removed because it was shutdown by Microsoft. Now you must set up authentication to use `aspeak`.
- The default voice list API url has been removed for the same reason.
- The rust API has been changed.
  - `Synthesizer` is now `Send`. Its various `synthesize_*` methods now takes `&mut self` instead of `&self`.
  - Now you need to use the builder pattern to create various options like `TextOptions`.
  - Fields of the `Voice` struct are now private. You can use the methods to access them.

## Other changes

- The PKGBUILDs for Arch Linux is no longer stored in this repository. You can find them in the [AUR](https://aur.archlinux.org/packages/aspeak).

# v4.3.1

- Fix a bug that caused the `endpoint` and `region` settings in profile to be ineffective.

# v4.3.0

- Add support for http and socks5 proxy. Command line option `--proxy` and environment variable `http_proxy`(or `HTTP_PROXY`) are available.
  - Example: `aspeak --proxy "socks5://127.0.0.1:7890" text "Hello World"`
  - You can also set the proxy in the `auth` section in your profile.
  - By now, connection to https proxy server is not supported!
  - For python binding, use the `proxy` keyword argument in the `SpeechService` constructor.
- Fix: Now the `list-voices` command correctly handles the auth settings. (region, token, key)
- Now you can specify the voice list API url when using the `list-voices` command.

# v4.3.0-beta.2

- Change the implementation of socks5 proxy.
- Make the `list-voices` command respect the proxy settings.
- Fix: Now the `list-voices` command correctly handles the auth settings. (region, token, key)
- Now you can specify the voice list API url when using the `list-voices` command.

# v4.3.0-beta.1

- Add support for http and socks5 proxy. Command line option `--proxy` and environment variable `http_proxy`(or `HTTP_PROXY`) are available.
  - Example: `aspeak --proxy "socks5://127.0.0.1:7890" text "Hello World"`
  - You can also set the proxy in the `auth` section in your profile.
  - By now, connection to https proxy server is not supported!
  - For python binding, use the `proxy` keyword argument in the `SpeechService` constructor.

# v4.2.0

- Show detailed error message in python bindings.
- Fix: Previously, the `role` field in the default profile template is not commented out and set to `Boy`.
  You might want to comment it out if you are already using the default profile template and haven't changed it.
- The `role`, `style` and `style_degree` fields are now commented out in the default profile template.
- Feature: Now you can use `--no-rich-ssml` flag to disable rich SSML features such as `role`, `style` and `style_degree`.
  This is useful if you are using an endpoint that does not support rich SSML features.
- Fix(Python bindings): Now the `SpeechService` constructor correctly takes an iterable instead of an iterator for `headers` keyword argument.
- Fix: Now aspeak correctly handles endpoint urls that contain query parameters.

# v4.1.0

- You can now use your azure subscription key to authenticate. Special thanks to [@yhmickey](https://github.com/yhmickey)
  for trusting me and providing me his subscription key for testing.

# v4.0.0

aspeak has been rewritten in Rust!:tada: This is a major release and there are some breaking changes.
Please read the documentation carefully if you are upgrading from v3.x.

Fixes:

- In some cases, the old aspeak can't play audio on some linux platforms.
- aspeak now respects the encoding arg for both stdin and file.
- Stricter validations for command line options.
- Do not overwrite existing file unless --overwrite is specified.

New features:

- Now you can use profiles to save your options.
  - For example, you can specify your native locale in your profile so that you don't need to specify it every time.
  - You can learn more about profiles in the [documentation](https://github.com/kxxt/aspeak/tree/main#documentation).
- Theoretically, aspeak is now available on more platforms. But I will only publish binaries for Windows, macOS and Linux.
  - However, you can still compile aspeak from source on other platforms.
- Now you can use custom endpoints and authentication tokens.
- Now you can add custom request headers.
- More user friendly output and error messages
- Now I have set up GitHub Actions to build and publish automatically.
- Now you can use aspeak as a library in your Rust projects.
  - You can learn more about the Rust API at [docs.rs](https://docs.rs/aspeak/).

Changes:

- RIIR
- We no longer publish linux wheels to PyPI because of manylinux compatibility issues. Please compile from source if you want to use python bindings on linux.
- The python bindings has been changed dramatically. Please read the documentation carefully if you are upgrading from v3.x.
- The `-F/--format` option now takes kebab case values(e.g. `audio-16khz-128kbitrate-mono-mp3`) instead of pascal case values.

# v4.0.0-rc.1

- Update docs.
- Fix a typo in config template.
- Internal refactor.
- Fix: Now you can use `-` to read from stdin.
- Fix: Correct some CLI help messages.

# v4.0.0-beta.4

- We no longer publish linux wheels to PyPI because of manylinux compatibility issues. Please compile from source if you want to use python bindings on linux.
- Improve the python bindings.
- Update docs.
- Automatically publish to crates.io.

# v4.0.0-beta.3

- Fix: include aspeak binary in wheel package
- CI: set up GitHub Actions to build and publish to PyPI

# v4.0.0-beta.2

- Restrict max log level to info in release build
- Update config template
- Do not overwrite existing file unless --overwrite is specified
- Revert to native-tls to reduce binary size
- Fix: Correctly handle quality setting from profile
- Fix: RequestID now gets reset on each request
- Internal refactor

# v4.0.0-beta.1

- Now you can create a profile!
  - You no longer need to use the locale arg every time if you do not want to use English locale. Just specify your locale in your profile.
  - You can also provide default values for other options.
- Many internal refactors.

# v4.0.0-alpha.4

- Get rid of openssl completely.

# v4.0.0-alpha.3

- Set up GitHub Actions to build for more platforms.
- Support region option.
