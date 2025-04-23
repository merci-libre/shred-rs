# shred-rs
A Rewrite of the `shred` [CoreUtil](https://github.com/coreutils/coreutils/blob/master/src/shred.c). Rewritten in Rust.

Still rewrites the entire sector by default (4kiB|4096B), see below for changes. 

Build with `cargo --build release`.

Differences between Original and Rust Rewrite:
- Original still utilizes less resources (see next point)
- All bytes inputted are generated randomly, increasing CPU utilization.

Report all bugs inside of issues.

## Notable improvements and trade-offs
### TO-DO's
-  Intensive CPU utilization (Wouldn't run this on an old microcontroller, but my Rasp Pi did fine when running this...)
-  No file-renaming on removal (To be added)
-  Missing key features (advanced file removal, byte sourcing from external files, file permissions, etc)

These features are planned to be adjusted or added with future optimizations and versions.

### Improvements
- 1.92x faster than the original (from my testing, this may vary from machine to machine)
- Multithreading and concurrent process handling.

and more to come!

## Philosophy and Mission

With the announcement from Canonical to abandon the Original GNU coreutils for Uutils in March of this year (2025), many people felt concern of the switch in licensing from the Gnu Public License to the MIT license. 
This in-turn reduced the amount of freedom and coverage of liability that the GPL-3 for many decades has been provided to the users of the GNU coreutils. My project and aim is to preserve the identity of the 
coreutils, and carry the legacy of the GPL into a new era for both Linux and free (as in libre) software. Of course with that said, you are not only allowed, but I advocate for you to make any contributions, modifications,
or redistribution of this as you see fit. It is not only a privilege that you are granted, but a right that involved years and years of lawsuits and litigations for them to exist. While many have their own opinions of Richard Stallman,
even within the FSF, his greatest contribution of the GPL license made Linux as we know it today possible.

Fundamentally, the coreutils should function as they always had in the preceeding iteration. Albeit this project is still in it's infancy, meaning that there are more optimizations and features to come. It is without a doubt that some
people are concerned over the use and 'hype' surrounding Rust, but the amount of memory security and language modernization is undeniable. I understand the concern that many people have when it comes to Rust, particularly in the 
Linux community, but Rust has proven time and time again its value. 

I didn't include this section in the original manuscript, but I feel as if it is an important message that I wanted to get off my chest. I think that you should be free to choose what you install on your system and that you have a right to
choose one software or the other, as needs vary from person to person. I understand that not many people will read this part of the `readme.md`, but it serves as both an ideological and philosophical reasoning as to where this project is heading.
Every project that I have released or created almost always used the GPL-3 license because fundamentally I believe in the core values of it. An end-user should not have to worry whether or not the software they have on their computer is malicious,
or have to suffer litigation for reverse engineering a project because fundamentally that software is **YOURS** to own. Ethically speaking, if I give you a chocolate bar to keep and own it is yours. You can eat it, you can sell it, you could even break it apart and 
make it into cookiedough if you like. The same can be said about software, and that's what the Gnu Public License gave to us. I shouldn't be able to sue you because you threw the chocalate bar I gave you into your fondue machine, that would be silly.

Anyways, any, and all contributions are welcome, feel free to fork the repository if you wish to make any other additions to this program.

-Westwardfishdme
