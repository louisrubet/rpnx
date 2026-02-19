# Changelog

The format of this file is based on [Keep a Changelog].

This project adheres to [Semantic Versioning].

All notable changes to this project are documented in this file.

## [1.0.0] - 2026-01-31

- `rpnx` is a porting of [rpn v2.5.0] to rust.
- This new name (project and executable) is chosen to avoid clashing with existing deb, rpm, flatpak, snap, or crate packages.
- Version re-starts from 1.0.0.

The list below shows the changes compared to rpn v2.5.0.

### Added
- Syntax coloring
- New commands `error` and `strerror` giving the last error encountered (integer and string)
- New bitwise operators `& \| ^ ~`
- Command `test` can take a symbol as argument
- `im` and `re` now work on reals

### Changes incompatible with rpn v2.5.0
- Command `round` is removed
- Command `%CH` is renamed `%inv`
- Operator `^` now designates bitwise xor and is not a short term for `pow` anymore
- `hex`, `bin` and base power input are removed, a syntax like `0x10p3` is no more supported
- Operation representation inheritance: left side representation is inherited, ex: `0b1111 0x23 +` will be shown as a binary
- Command `->str` on symbols does not anymore include `'` in the string. Ex: `'one' ->str` gives `"one"`, not `"'one'"`
- `prec` command minimum value is now 2 instead of 0
- Negative signed zero `-0` does not exist anymore as an output

### Changed
- History file location now follows XDG Base Directory specification
  - New location: `$XDG_DATA_HOME/rpnx/history` (defaults to `~/.local/share/rpnx/history`)
  - Directory is created automatically if it doesn't exist
- Help text improvements
- New `README.md`

### Fixed
- Command `test` now works

[Keep a Changelog]: (https://keepachangelog.com/en/1.0.0/)
[Semantic Versioning]: (https://semver.org/spec/v2.0.0.html)
[1.0.0]: https://github.com/louisrubet/rpnx/releases/tag/v1.0.0
[rpn v2.5.0]: https://github.com/louisrubet/rpn/releases/tag/v2.5.0
