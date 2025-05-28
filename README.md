# Rune Language for Zed

This is a Zed extension that provides language support for the Rune programming language.

Currently, the extension is based on tree-sitter rust parser and zed build-in rust tree-sitter queries.

While the syntax highlighting works correctly, this could cause issues in outline view and some other features that rely on tree-sitter queries. Rust parser expects type identifiers present, thus causing errors in AST.

I have a tree-sitter parser for Rune ready at https://github.com/zhuhaow/tree-sitter-rune, but a tree-sitter queries for it is not ready yet.

## License

MIT License
