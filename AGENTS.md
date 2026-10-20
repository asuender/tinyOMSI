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

The inherited website, Pages publishing, governance and issue/PR policy bots have
been removed. Docker server packaging and its container-based bus-sync test helper
have also been removed; native server code and build jobs remain pending removal.
Android runtime support, packaging, launcher web-view integration and CI builds have
also been removed. iOS/iPadOS, macOS and desktop touch controls remain. Keep legacy
Android signing keys and build output ignored. Shared phone settings remain for iOS;
no settings keys or settings paths were changed by the Android removal.
The upstream player-count heartbeat, its Cloudflare Worker and count-me settings have
been removed. Discord activity sharing and Steamworks integration (including the overlay)
have also been removed. Old `presence`, `discord_status` and `discord_app_id` settings are
ignored and dropped on save. Steam installation discovery, Steam Deck support and
multiplayer heartbeats remain.
The `omsi-mt` machine translation crate (NLLB-200 via CTranslate2) and its `omsi-ui`
fallback hook have been removed; interface texts come only from the translation tables.
Old `machine_translation` settings are ignored and dropped on save.
Build, security, crate dependency-boundary and locale checks remain. Source
line-count limits and the size-budget script have been removed.
The CodeQL, ray tracing check, PR test build comment and beta promotion workflows have
been removed; a beta is promoted by hand (see `docs/VERSIONING.md`). Pull requests and
pushes to `main` build only Windows x64, macOS arm64 and Linux x64; ARM64 Windows/Linux
and Intel macOS are built only by a manual release run.

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

## Remote development workflow

Edit locally, sync with Crabbox, and run checks on Daytona. Fix failures locally
and repeat until the relevant checks pass. Sync includes uncommitted changes;
no commit or push is needed. Do not compile or run tests on the local MacBook,
including when remote access fails.

Run Crabbox from the repository root. The ignored `.crabbox.yaml` connects to
`tinyomsi2`; approve its SSH host explicitly as shown below. If the sandbox is
stopped or its SSH token has expired, ask the user to start it or refresh the
token. Session tokens in Crabbox output need no manual redaction. Never commit
credentials or copy them into tracked files.

- Edit and format source locally. Remote edits do not sync back automatically.
- Exclude `.crabbox.yaml`, secrets, proprietary content and build outputs from
  sync. Keep credentials ignored by Git too.
- Serialize each sync-and-check run in the shared workspace. Use separate
  workspaces for parallel agents so one sync cannot overwrite another's inputs.
- Preserve `target/` and `launcher/src-tauri/target/`. Do not routinely clean
  caches or use `--full-resync`. Build on sandbox local disk, not the S3-backed
  volume, and monitor free space.
- `.cargo/config.toml` limits builds to four jobs and disables incremental
  compilation and dev/test debug symbols to conserve disk. Run Cargo from the
  repository root so these settings also apply to the separate launcher workspace.
- `--keep` retains the Crabbox lease, not control of Daytona's power or billing.
  Start and stop the sandbox through Daytona.

## Verification

Run checks through Crabbox and report what actually ran. For full workspace
verification:

```sh
crabbox run --static-host ssh.app.daytona.io --keep -- sh -c '
  set -eu
  . "$HOME/.cargo/env"
  cargo fmt --all -- --check
  cargo check --locked --workspace
  cargo test --locked --workspace
'
```

Use targeted crate checks and tests while iterating, then broader checks before
finishing. Check the launcher separately with
`cargo check --locked --manifest-path launcher/src-tauri/Cargo.toml` in the same
Crabbox wrapper. Keep `set -e` or explicitly propagate failures; a successful sync
does not mean checks passed. Report unrelated baseline failures without making
out-of-scope fixes.

Content-dependent checks require a local OMSI 2 installation via `OMSI_ROOT`.
Tests skipped without content do not establish content compatibility.

Check launcher changes separately; root workspace checks do not cover the
excluded Tauri launcher. For gameplay or rendering changes, include a manual
driving check when a local installation is available.

Preserve the crate dependency-boundary checks in `scripts/arch-guard.sh` and
`scripts/arch-rules.txt`. Do not reintroduce physical line-count limits.
Documentation must distinguish current behavior from planned removals. If new
information comes along, update `README.md` and `AGENTS.md`.
