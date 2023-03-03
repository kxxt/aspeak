# aspeak Changelog

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