# Tescore

A terminal based grading sheet inspector. Rewritten in Rust

## Dependencies

* uuid              : https://crates.io/crates/uuid
* chacha20poly1305  : https://crates.io/crates/chacha20poly1305
* pancurses         : https://crates.io/crates/pancurses
* crossterm         : https://crates.io/crates/crossterm

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