# unindent

Unindent is a command-line utility written in Rust that removes leading white spaces from text read from stdin. The goal of this utility is to make it easier to work with indented text by removing the leading whitespaces and making the text more readable.

## Installation

You can install the unindent-tool by using cargo:

```bash
cargo install unindent
```

## Usage

You can use `unindent` by piping text into it:

```bash
echo "    Indented text" | unindent
```

This will output:


```bash
Indented text
```

You can also use `unindent` as part of a larger pipeline:

```bash
cat indented-file.txt | unindent | some-other-command
```

## Contributing

Contributions are welcome! If you'd like to contribute to ```unindent``` project, please open an issue or a pull request.

## License

This project is licensed under the [MIT License](https://opensource.org/licenses/MIT).
