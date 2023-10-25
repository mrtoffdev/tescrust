## Tescrust
A terminal-centric tabulator & data processing engine with a customizable responsive TUI & hotkey bindings, batteries included. Second to-be-released version of the Tescore series. Written in Rust and will support windows & linux-based systems

---
### 📢 Warning: This is a work in progress, and is by no means a functional tool yet. Currently working on developing a TUI library using crossterm alongside this project, which is what's mostly occupying development time, and will most likely receive its own repository in the future 📢

## Dependencies
* [uuid](https://crates.io/crates/uuid)
* [chacha20poly1305](https://crates.io/crates/chacha20poly1305)
* [crossterm](https://crates.io/crates/crossterm)

## Roadmap
- [ ] IO implementation
  - [ ] Safeguard Lib:
    - [ ] Replace AES128 w/ XChaCha20Poly1305
    - [ ] Cache flushing
  - [ ] Misc:
      - [ ] Replace full overwrites w/ diff-based r/w operations
      - [ ] Caching
      - [ ] Event Undo / Redo
- [ ] TUI implementation:
  - [ ] Replace raw Stdout calls w/ Crossterm 
  - [ ] Replace hardcoded UI element props w/ dynamic & responsive solution
    - [ ] Color Scheme
    - [ ] Word Wrap Limits
    - [ ] Component Layout
- [ ] Nav implementation:
  - [ ] Replace hardcoded bindings w/ config file
    - [ ] Embeddable configs
    - [ ] Global configs
  - [ ] Replace god forbidden `getch()` calls
- [ ] Data models:
  - [ ] Refactor data model to include non-int Orderables
  - [ ] Event History
