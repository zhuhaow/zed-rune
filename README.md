# Rune Language for Zed

This is a Zed extension that provides language support for the Rune programming language.

Currently, the extension is based on tree-sitter rust parser and zed build-in rust tree-sitter queries.

This could cause issues in outline view and some other features that rely on tree-sitter queries since rust parser expects type identifiers presents.

While I have a tree-sitter parser for Rune ready at https://github.com/zhuhaow/tree-sitter-rune, I don't have time to write tree-sitter queries for it yet.

## License

MIT License
