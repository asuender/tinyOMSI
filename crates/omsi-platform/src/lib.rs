//! openOMSI's platform services: what the game and the launcher do with the world outside
//! the simulation - the updates from GitHub (`updater`),
//! what the start guard needs to heal a broken release (`recovery`) - and the version stamp
//! of the build.
//!
//! Nothing here knows the game's state: the app hands over plain values (a status line, a
//! version) and shows what comes back.

pub mod avatars;
pub mod memory;
pub mod proxy;
pub mod quit;
pub mod recovery;
pub mod support_bundle;
pub mod tomorrow;
pub mod updater;
pub mod webview2;

/// The commit the build came from, with `+` when the tree had changes, and its date (see
/// `build.rs`).
pub const BUILD: &str = env!("OMSI_BUILD");

/// The release version, `MAJOR.MINOR.PATCH` (see `build.rs` and docs/VERSIONING.md).
pub const VERSION: &str = env!("OPENOMSI_VERSION");
