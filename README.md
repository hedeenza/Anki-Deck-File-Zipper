# Anki Deck File Zipper

## Use Case:
- When creating decks for Anki, I would often find myself with one file containing sentences in a target language and another file of their translations with no easy way to "horizontally" paste them together. Formatting them such that they could be easily imported, where line one from the target language file and line one from the translations file were on the same line, would have potentially taken hours of manual copy-and-pasting. This tool was created to save that time.
- This program could also be used on the output file to add additional tab-separated columns.

## Get the Tool
- The pre-compiled binary and source code are available in "Releases".

## Running the CLI
`$ anki_deck_file_zipper <Target_Language_File> <Base_Language_File> <Output_File>`

- Target Language = File containing the sentences in the target language
- Base Language = File containing the translations in a language you understand (e.g. your native language)
- Output = Name of the combined file produced by the program
- Ensure the file-zipping program has executable permissions.
- Ensure the number of lines in the <Target_Language_File> and the <Base_Language_File> are the same. "Trailing" lines will be removed.

## Building from Source
- If using cargo: `$ cargo build --release`
- If not using cargo: `$ rustc -0 src/main.rs`

## Running the CLI from anywhere in your file system
Add the following lines to your `.bashrc` file:
```
~/.bashrc
# Anki Deck File Zipper
export PATH="$PATH:/path/to/directory/where/this/program/lives"

alias az="anki_deck_file_zipper"
```
