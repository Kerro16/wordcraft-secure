# Security hardening of this fork

Based on upstream `storytold/wordcraft` at commit d859c67.

## Changes
- Removed the `craft-update` dependency (external git crate `storytold/craft-libs`): the app no longer
  checks for, downloads or installs updates, and makes no network requests at startup.
- Removed the Windows `--finish-update` self-update helper from `apps/wordcraft/src/main.rs`.
- `crates/ui-egui/src/updates_ui.rs` is now a stub; Help > Check for Updates only shows a notice.

## Not yet done
- This fork has not been compiled or tested yet (no Rust toolchain was available when it was made).
  `Cargo.lock` still lists `craft-update`; `cargo build` will prune it.
- Remaining git dependencies (`craft-libs`: craft-pdf, craft-print, krilla; forks of egui, winit,
  winprint.rs) are pinned to exact commits but their code has not been audited.
- Run `cargo audit` and verify with a network monitor that the built binary makes no outbound connections.
- The optional local control server (`--control`, 127.0.0.1 + random key) is unchanged and off by default.
