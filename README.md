# shred-rs
A Rewrite of the `shred` CoreUtil. Written in Rust.

Still rewrites the entire sector by default, see below for changes. 

Build with `cargo --build release`.

Differences between Original and Rust Rewrite:
- Original still utilizes less resources (see next point)
- All bytes inputted are generated randomly, increasing CPU utilization.

Report all bugs inside of issues.
