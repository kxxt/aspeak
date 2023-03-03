# aspeak Changelog

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