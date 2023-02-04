# unindenter

Unindenter is a command-line utility written in Rust that removes leading white spaces from text read from stdin.
The goal of this utility is to make it easier to work with indented text by removing the leading whitespaces and making the text more readable.

## Installation

You can install the unindenter by using cargo:

```bash
cargo install --locked unindenter
```

## Usage

You can use `unindenter` by piping text into it:

```bash
echo "    Indented text" | unindenter
```

This will output:


```bash
Indented text
```

You can also use `unindenter` as part of a larger pipeline:

```bash
cat indented-file.txt | unindenter | some-other-command
```

## Contributing

Contributions are welcome! If you'd like to contribute to ```unindenter``` project, please open an issue or a pull request.

## License

This project is licensed under the [MIT License](https://opensource.org/licenses/MIT).
