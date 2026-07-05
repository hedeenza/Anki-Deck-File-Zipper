# Anki Deck File Zipper

## Use Case:
- When creating decks for Anki, I would often find myself with one file containing sentences in a target language and another file of their translations with no easy way to "horizontally" paste them together. Formatting them such that they could be easily imported, where line one from the target language file and line one from the translations file were on the same line, would have potentially taken hours of manual copy-and-pasting. This tool was created to save that time.
- This program could also be used on the output file to add additional tab-separated columns.

## Get the Tool
- Source code is available in "Releases".

## Running the CLI
`$ ./anki_deck_file_zipper -t <Target_Language_File> -n <Native_Language_File> -o <Output_File>`

- `--target-language, -t <TARGET_LANGUAGE>`: The file containing the sentences in the target language
- `--native-language, -n <NATIVE_LANGUAGE>`: The file containing the translations in a language you understand (e.g. your native language)
- `--output-file, -o <OUTPUT_FILE>`: The name of the Output File
- `--help, -h`: Print help
- Ensure the file-zipping program has executable permissions.
- Ensure the number of lines in the <Target_Language> file and the <Native_Language> file are the same. "Trailing" lines will be removed.

## Building from Source
Navigate to the root directory.
- If using cargo: `$ cargo build --release`
- If not using cargo: `$ rustc -0 src/main.rs`

The executable binary should then be available in `./target/release/`

## Running the CLI from anywhere in your file system
Add the following lines to your `.bashrc` file:
```
~/.bashrc
# Anki Deck File Zipper
export PATH="$PATH:/path/to/directory/where/this/program/lives"

alias az="anki_deck_file_zipper"
```

## License

