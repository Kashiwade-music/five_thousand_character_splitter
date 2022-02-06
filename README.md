# 5000 character splitter

This is a Rust practice project

## About
This program splits the text in a given .txt file into 5,000-character chunks and outputs them. It can be used, for example, when you want DeepL to translate a long piece of text.

## How To Use
1. Build this program
    ```sh
    cargo build --release
    ```
1. Lanch
    ```sh
    # in Windows
    five_thousand_words_splitter.exe ./test.txt
    ```
1. Enjoy!
   - The split file will be output as "output_xx.txt".

## Other
It will be split every 5000 characters, but at the position of the space and period closest to the 5000th character.

## Example
`test.txt` is a sample input text file.
`output_test.txt` is a sample output text file.