# RangerKit Hackathon Trail Map

🌲 A gentle trail guide for the first RangerKit hackathon

---

## Phase 1 — Set up the RangerKit workspace
- [x] Create the `rangerkit/` root folder
- [x] Create the `Cargo.toml` workspace file
- [x] Scaffold the initial crates inside `crates/`:
  - [x] `rangerkit-cli`
  - [x] `rangerkit-core`
  - [x] `rangerkit-spirits`
- [x] Set up basic `main.rs` and `lib.rs` files
- [x] Move workspace structure to use `crates/` folder
- [x] Fix initial build issues (nested git repos, etc.)

---

## Phase 2 — Build the Spirit Manifest
- [x] Define `Ability` struct (with `new()` constructor)
- [x] Define `Spirit` struct (with `new()` and `with_ability()`)
- [x] Define `SpiritManifest` struct (with `new()` and `with_spirit()`)
- [x] Implement detailed Rust docstrings everywhere
- [x] Default manifest containing:
  - [x] Lantern Spirit
  - [x] Waystone Spirit

---

## Phase 3 — Create the CLI
- [x] Add `clap` CLI parsing to `rangerkit-cli`
- [x] Create `spirits` subcommand
- [x] Create `commune` sub-subcommand
- [x] Print spirit manifest in a friendly text table
- [x] Add `--format json` flag
- [x] Output structured JSON for piping into Nushell
- [x] Colorize CLI output for text mode
- [x] Prepare to migrate text coloring to `nu-ansi-term`

---

## Phase 4 — Devlogs, Checklist, and Trail Markers
- [x] Create `devlogs/` directory
- [x] Write `0000-hackathon-start.md`
- [x] Write `0001-first-spirits-commune.md`
- [x] Scaffold `0002-summon-first-spirit.md`
- [x] Create `CHECKLIST.md`
- [x] Create `CHANGELOG.md`
- [x] Add first Git tags:
  - [x] `v0.0.1-trailhead`
  - [x] `v0.0.2-summoning`

---

## Phase 5 — Book of RangerKit (mdBook)
- [x] Install and initialize `mdBook`
- [x] Create `/book/` structure
- [x] Write `index.md` (Welcome to RangerKit)
- [x] Write `spirits.md` (Meeting the Spirits)
- [x] Write `cli-guide.md` (Walking the Trails)
- [x] Serve and view the Book locally
- [x] Prepare future chapters:
  - [ ] Spirit Whispers (planned)
  - [ ] Future Trails (planned)

---

# Future Trails (Planned)
- [ ] Integrate `nu-ansi-term` for richer CLI coloring
- [ ] Add random Spirit Whispers during commune and summon
- [ ] Build a `spirits invoke` system for abilities
- [ ] Sketch the Waystone Star Map TUI
- [ ] Implement Listening Stones (background watchers)
- [ ] Document CLI options and spirit philosophy in the Book

---

# Notes
- The spirits have gathered.
- The trails are open.
- The RangerKit Book breathes.
- The forest listens.

---

# Ranger's Blessing
> *The first stones are laid.  
> The first companions walk beside you.  
> The forest has already begun to remember.*

