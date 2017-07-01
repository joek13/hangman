# Hangman
A terminal-based hangman game I wrote.

Super hacky. Code is sorta messy, and there's no commit history, but I figured I'd put it up since it's kinda fun.

No way to lose right now, and words are sorta weird (I ran the wordlist through a script to remove ones that weren't alphabetic, but some of the words are sort of uncommon).

## Running
------

Clone the repo and run with `cargo run`. Make sure you have Rust installed, of course.

This won't work on Windows, as it is dependent on Termion (a Unix-only crate). Sorry bout that.
