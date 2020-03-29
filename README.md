
This repository holds the code to generate puzzles for printing on A4 paper.


Creating new puzzles
---------------------

- [Install rust langauge](https://www.rust-lang.org/tools/install)
- Create a new folder in `puzzles` with the name of your puzzle
- Add a `README.md` file with information on where the images are from, to verify that they are licensed properly to use as a public available puzzle.
- Add 16 images in the folder: 8 ending with `_fst.png` (first) and 8 ending with `_snd.png` (second). The first and second images will form pairs on the puzzle page.
- Execute `cargo run` inside the project folder, this will generate a `.pdf` file with the same name as your puzzle directory.

If you know how, please add the puzzle to this github repository and/or open a pull-request. You can also simply email me a zip with the images.

