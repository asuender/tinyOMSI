# tinyOMSI

A personal, independent fork of [openOMSI](https://github.com/openOMSI-org/openOMSI),
the Rust recreation of OMSI 2.

**Let it be minimal, but make it work.**

tinyOMSI aims to preserve the original driving experience and compatibility with
existing OMSI 2 content while reducing features, configuration and maintenance.

## Direction

- Focus on single-player driving.
- Keep one UI design instead of selectable styles and layouts.
- Remove Enhanced and Enhanced+ graphics modes.
- Preserve maps, buses, scripts, timetables, weather, sound and controller support.
- Incorporate upstream improvements where they fit these goals.

These are ongoing goals. The code still contains upstream features scheduled for
removal. The inherited website, repository policy bots and Docker server packaging
have been removed; multiplayer runtime code remains. Expect bugs and incomplete
functionality.

Android support has been removed. iOS/iPadOS and desktop touch controls remain; broader
mobile removal is still planned. Existing settings files are unchanged.

Crate dependency-boundary checks remain; formatting-sensitive source line-count
limits have been removed.

## Requirements

An installed copy of **OMSI 2** is required. tinyOMSI contains no original game
content and loads maps, vehicles and other assets from your installation.

## Building

Clone the fork:

```sh
git clone https://github.com/asuender/tinyOMSI.git
cd tinyOMSI
```

Use the build script for your platform:

| Platform | Command |
| --- | --- |
| macOS | `scripts/build-macos.sh` |
| Linux | `scripts/build-linux.sh` |
| Windows | `scripts\build-windows.cmd` |

Rust stable and the platform's native build tools are required.
See [Building](docs/BUILDING.md) for dependencies and detailed instructions.

Executables, packages and settings paths currently retain upstream names such
as `openomsi`. Renaming them is separate work.

## Documentation

- [User guide](docs/USER_GUIDE.md)
- [Building](docs/BUILDING.md)
- [Architecture](docs/ARCHITECTURE.md)
- [Content formats](docs/FORMATS.md)

The documentation is inherited from openOMSI and has not yet been fully adapted.
It may describe upstream features, release channels and services rather than
tinyOMSI. Do not assume the inherited updater targets this fork.

Report fork-specific issues in
[tinyOMSI's issue tracker](https://github.com/asuender/tinyOMSI/issues).

## Credits and license

tinyOMSI builds on the work of the openOMSI authors and contributors.
The original openOMSI codebase was created by usonskyyyy.
The original project is maintained at
[openOMSI-org/openOMSI](https://github.com/openOMSI-org/openOMSI).
Upstream also acknowledges contributions and public work from
[neoOMSI](https://github.com/neoOMSI/neoOMSI).

Released under [AGPL-3.0-only](LICENSE). Historical MIT notices and third-party
licenses remain applicable. The launcher uses
[jpeg-encoder](https://crates.io/crates/jpeg-encoder), which is based in part on
the work of the Independent JPEG Group.

The openOMSI name and original logos belong to their respective owner;
see [TRADEMARKS.md](TRADEMARKS.md). tinyOMSI is not an official openOMSI release.
OMSI and OMSI 2 belong to their respective owners; this project is not affiliated
with or endorsed by them.
