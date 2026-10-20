# Versioning and releases

## The version number: `MAJOR.MINOR.PATCH`

| Part | Where it comes from | Example |
| --- | --- | --- |
| `MAJOR` | the first number in the [`VERSION`](../VERSION) file, changed by hand | `1` |
| `MINOR` | the second number in `VERSION`, changed by hand | `1.2` |
| `PATCH` | counted automatically: one more than the last release `v<MAJOR.MINOR>.*` (its tag), or `0` when there is none yet | `1.2.17` |

Releases follow each other without gaps, however many commits come in between: after
`1.2.16`, the next release is `1.2.17`. A commit that already has a release keeps its version.
`PATCH` restarts from `0` whenever `VERSION` changes: after `1.1.23`, the first release with
`VERSION` set to `1.2` is `1.2.0`, and the next one `1.2.1`.

To start a new version, change that one line in a pull request like any other change:

```sh
git switch -c version-1.2 && echo 1.2 > VERSION
git commit -am "Version 1.2" && git push -u origin version-1.2
```

`scripts/version.sh` prints the version of the checked-out commit; the game shows it in the
launcher's side bar, in its log and in `openomsi --version` (baked in by
`crates/omsi-platform/build.rs`, which uses the same rule). A build of a commit that has no
release shows the number of the next release, whichever commit that release comes from.

## Releases: beta, then stable

There are two channels:

| Channel | What it is | Who gets it |
| --- | --- | --- |
| **Beta** | a build of `main` a maintainer starts by hand, published as a GitHub **pre-release** | testers: the launcher's **Settings → General → Updates → Update channel: Beta**, or the [Releases](https://github.com/openOMSI-org/openOMSI/releases) page |
| **Stable** | a beta the maintainers have tested and promoted | every other player: on the Stable channel (the default) the launcher's updater, the start guard, the website's Download page, `releases/latest` and the README badge only ever see stable releases |

On the Beta channel the updater offers the newest release, beta or stable, that has a file
for this computer and is not listed in `release/health.json`, and *Choose a version…* lists
the betas too (marked *Beta*). Changing the channel looks for an update again at once. Back on Stable, nothing is taken away: the
beta installed stays until a newer stable release comes out, or an older one is chosen in
*Choose a version…*.

### Beta: a release started by hand

A push to `main` publishes nothing by itself. A maintainer starts a release with **Actions →
Release → Run workflow** from `main` (or `gh workflow run release.yml --ref main`), which runs
[`.github/workflows/release.yml`](../.github/workflows/release.yml) for the commit `main` is on:

1. works out the version with `scripts/version.sh`;
2. builds Windows x64 and ARM64 (MSVC), macOS for Apple silicon and Intel, Linux x64 and
   ARM64 in parallel, and packs the dedicated server from the Linux and Windows
   builds;
3. creates the pre-release `v<version>` (tag on that commit) with the archives
   `openOMSI-<version>-windows-x64.zip`, `-windows-arm64.zip`, `-macos-arm64.zip`,
   `-macos-x64.zip`, `-linux-x64.zip`, `-linux-arm64.zip`,
   `-server-linux-x64.zip`, `-server-linux-arm64.zip`, `-server-windows-x64.zip`,
   `-server-windows-arm64.zip`, the installers `-windows-x64-setup.exe`,
   `-windows-arm64-setup.exe`, `-macos-arm64.dmg` and `-macos-x64.dmg`, a `SHA256SUMS`
   file with the SHA-256 of each of them, and
   release notes from this version's section of `CHANGELOG.md`
   ([`scripts/release-notes.sh`](../scripts/release-notes.sh)).

### Stable: promoting a tested beta

Only betas of `main` are published (a workflow run on another branch builds without
publishing). tinyOMSI has no promotion workflow. A tested beta becomes the stable release
by hand, as it is: the same tag, the same files, the same SHA-256, nothing built again.
Before promoting, check that it is a published beta of `main`, newer than the current stable
release, has the files of every platform and is not listed in `release/health.json`. Then
rewrite its notes to cover every change since the previous stable release:

```sh
sh scripts/release-notes.sh <version> <previous-stable> > notes.md
gh release edit v<version> --prerelease=false --latest --notes-file notes.md
```

### A published release never changes

A version is published once. The release job refuses to publish a version that already
exists, whose tag names another commit, or whose commit is older than the last release; it
never moves a tag nor replaces a file. A release's tag is never deleted either: the next
version is counted from the tags. A mistake in a release is fixed by a new commit, hence a
new version. With the repository's
*immutable releases* setting on, GitHub enforces it too.

The `SHA256SUMS` of a release checks a download by hand:

```sh
sha256sum -c SHA256SUMS --ignore-missing
```

The launcher's updater checks each archive against the SHA-256 that GitHub's API lists for
it before installing it (when the API cannot be reached and the updater falls back to
github.com, it relies on the HTTPS connection alone).

Pull requests and pushes to `main` build Windows x64, macOS for Apple silicon and Linux
x64 without publishing anything; only a manual release run builds every platform above. Build output never goes into the repository (`target/` and `dist/` are ignored).

### Test builds of a pull request

A pull request's builds are test builds for anyone who wants to try the change:

* their version names the pull request, `<version>-pr<number>` (e.g. `0.1.1313-pr1192`),
  in the launcher's side bar, `game.log` and crash reports;
* the updater never offers them a release, so a test build stays until it is deleted;
* the builds are artifacts of the pull request's Release run, kept for 14 days.

The version badge at the top of the README always shows the newest stable release.

### A broken release

The start guard (`crates/omsi-guard`, see docs/ARCHITECTURE.md) heals a release that crashes
or hangs at the start on every player's computer by itself: a newer release is installed at
once, or the version before is put back. For a release that starts but is broken otherwise
(saves, physics, a crash a minute in), list it in [`release/health.json`](../release/health.json)
on `main`:

```json
{ "bad": ["0.2.31"], "message": "0.2.31 loses the timetable after a stop" }
```

Every start of a listed version notes it; the next start installs a newer release when there
is one, else goes back to the version before (when its files are still kept, i.e. the listed
version has not been running long), and tells the player why with the message. Then push the
fix and promote its beta as soon as it is checked. The list is read from GitHub with a short wait at each start; offline,
nothing happens. Take a version off the list once nobody runs it any more.

Before any update replaces a program, the new one must pass `openomsi --self-test` (it starts,
says its version and ends): a release that cannot even be loaded on a computer is refused there
and never offered again.


## The website

The inherited website and its GitHub Pages workflow have been removed from tinyOMSI.
Documentation remains in `docs/`. The release instructions above describe inherited
upstream tooling, not a verified tinyOMSI release process.
