# Contributing to tinyOMSI

Keep changes focused on a minimal, working single-player OMSI 2 recreation.
See [README.md](README.md) for direction and [AGENTS.md](AGENTS.md) for development
rules. Multiplayer and Enhanced graphics are still present pending removal.

## Changes

- Make one focused change per pull request. Explain why it is needed, what could
  break, and how it was verified. Include screenshots for visible UI changes.
- Preserve existing content compatibility and handle old settings explicitly.
- Follow existing code style. Avoid unrelated rewrites and dependency upgrades.
- Preserve architecture and size checks under `scripts/`.

Run checks appropriate to the change:

```sh
cargo fmt --all -- --check
cargo check --workspace
cargo test --workspace
```

Check the excluded Tauri launcher separately when it changes. Content-dependent
checks require `OMSI_ROOT`; skipped tests do not establish compatibility. Gameplay
and rendering changes also need a manual driving check. Report checks not run.

Build instructions: [docs/BUILDING.md](docs/BUILDING.md). Architecture:
[docs/ARCHITECTURE.md](docs/ARCHITECTURE.md). Formats:
[docs/FORMATS.md](docs/FORMATS.md).

## Content and licenses

- Never commit proprietary OMSI 2 code or content, downloaded models, caches or
  builds. Use a local OMSI 2 installation for content-dependent tests.
- New contributions are AGPL-3.0-only. Preserve historical MIT notices,
  third-party licenses and upstream attribution. Inter fonts and Material icons
  retain their own licenses.
- Keep test fixtures small and original. Files of a megabyte or more fail the PR
  checks; put screenshots and videos in the pull request description instead.

## Translations and issues

Interface translations live in `crates/omsi-app/locales/app.yml`. Preserve English
keys and placeholders such as `%{packs}` and `{}`; remove keys only when their
callers have been removed.

Report one problem or idea per issue in
[tinyOMSI's issue tracker](https://github.com/asuender/tinyOMSI/issues). Include the
commit or build version, platform, reproduction steps and relevant logs. Review
logs for personal information before posting. Do not post vulnerabilities
publicly; see [SECURITY.md](SECURITY.md), whose inherited reporting guidance has
not yet been adapted for this fork.
