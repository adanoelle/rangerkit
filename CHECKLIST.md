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

## Phase 4 — Devlogs and Bestiary
- [x] Create `devlogs/` directory
- [x] Create first devlog `0000-hackathon-start.md`
- [x] Create `spirits/SPIRITS.md` bestiary scaffold
- [x] Write lore entries for Lantern Spirit and Waystone Spirit

---

## Phase 5 — In Progress / Next Steps
- [ ] Integrate `nu-ansi-term` for cleaner colored output
- [ ] Create `spirits summon <spirit_name>` CLI
- [ ] Lookup and display summoned spirit details
- [ ] Add soft "whisper" messages during spirit summon
- [ ] Begin planning spirit abilities for action execution
- [ ] (Optional) Start the first Waystone Map TUI

---

# Notes
- The spirits have been summoned.
- The trailhead is marked.
- The first gathering has begun.

---

# Ranger's Blessing
> *The stars above, the stones below —  
> and now, the spirits beside you.*

