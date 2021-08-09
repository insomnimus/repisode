# Repisode

Repisode is a yet another episode number fixer.

It pads episode numbers (or any number!) with a `0` where required, making your files sort properly.

# Example Scenario

Lets say you have these files:

-	Rick and Morty S1E1.mp4
-	Rick and Morty S1E10.mp4
-	Rick and Morty S1E11.mp4
-	Rick and Morty S1E12.mp4
-	Rick and Morty S1E2.mp4
-	Rick and Morty S1E3.mp4
-	Rick and Morty S1E4.mp4
-	Rick and Morty S1E5.mp4
-	Rick and Morty S1E6.mp4
-	Rick and Morty S1E7.mp4
-	Rick and Morty S1E8.mp4
-	Rick and Morty S1E9.mp4

The files are sorted alphabetically, but the alphabetical order does not reflect the episode order! 
If you wanted to use `ffmpeg`, you probably would like to fix your file names first!

Here's where `repisode` comes in handy. In a single command your files will be properly ordered:

```sh
repisode 'Rick and Morty S1E@N@.mp4' *.mp4
```

> The `@N@` is required to specify the number that needs fixing, this is for a finer control.

Observe the output of `ls -1` after running the above:

```output
Rick and Morty S1E01.mp4
Rick and Morty S1E02.mp4
Rick and Morty S1E03.mp4
Rick and Morty S1E04.mp4
Rick and Morty S1E05.mp4
Rick and Morty S1E06.mp4
Rick and Morty S1E07.mp4
Rick and Morty S1E08.mp4
Rick and Morty S1E09.mp4
Rick and Morty S1E10.mp4
Rick and Morty S1E11.mp4
Rick and Morty S1E12.mp4
```

Wham, ready for concatenation.

# Installation

Repisode is written in rust and tested with `cargo version 1.55.0`.
You will need `git` and an up to date rust toolchain installed.

The preferred way of installation is by git clone:

```sh
git clone https://github.com/insomnimus/repisode
cd repisode
git checkout main
# to run tests; do:
# cargo test
cargo install --path .
```

The other method is using cargo directly:

`cargo install --git https://github.com/insomnimus/repisode --branch main`

Have fun, don't forget to check for updates!
