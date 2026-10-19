# Agent instructions

## Project

tinyOMSI is a personal, independent fork of openOMSI:
<https://github.com/openOMSI-org/openOMSI>

The goal is a minimal, working OMSI 2 recreation focused on single-player driving
and compatibility with existing content. Minification work currently happens on
the `minification` branch.

Planned changes include removing multiplayer, keeping one UI design, and removing
Enhanced and Enhanced+ graphics modes. Do not describe these as complete without
checking the implementation.

## Repository map

- `crates/`: Rust workspace; `omsi-app` integrates the game subsystems.
- `launcher/ui/`: launcher web interface.
- `launcher/src-tauri/`: native launcher, outside the root Cargo workspace.
- `scripts/`: builds, validation and release tooling.
- `docs/`: inherited technical and user documentation.
- `assets/`: project assets, not OMSI 2 game content.

Read `docs/ARCHITECTURE.md` before changing subsystem boundaries and
`docs/BUILDING.md` before changing build tooling.

## Development rules

- Read relevant code before proposing changes.
- Make small, reviewable changes with one clear purpose.
- Remove unwanted features end to end, not just their UI controls.
- Preserve single-player behavior and content compatibility.
- Check shared callers before deleting multiplayer or Enhanced code.
- Handle old settings explicitly when removing options.
- Avoid unrelated rewrites, formatting changes and dependency upgrades.
- Keep upstream updates manageable by separating removals from refactors.
- Use tinyOMSI in new user-facing documentation. Preserve upstream attribution,
  license notices and accurate technical names.
- Do not mechanically rename crates, binaries, settings paths or external URLs.
- Treat inherited release and updater destinations as unverified for this fork.
- Use minions only for read-only audits, never to make changes.
- Never commit proprietary OMSI 2 content, downloaded models, caches or builds.
- Keep prose concise and comments focused on intent. No emojis or em dashes.

## Verification

Run checks appropriate to the change and report what actually ran:

```sh
cargo fmt --all -- --check
cargo check --workspace
cargo test --workspace
```

Content-dependent checks require a local OMSI 2 installation via `OMSI_ROOT`.
Tests skipped without content do not establish content compatibility.

Check launcher changes separately; root workspace checks do not cover the
excluded Tauri launcher. For gameplay or rendering changes, include a manual
driving check when a local installation is available.

Preserve the architecture and size checks under `scripts/`.
Documentation must distinguish current behavior from planned removals. If new
information comes along, update `README.md` and `AGENTS.md`.
