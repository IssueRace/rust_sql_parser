# rust_sql_parser

A hand-written SQL parser built in Rust from scratch, without any external parser libraries.

## Features

- **Tokenizer** - lexical analysis that splits raw SQL input into a stream of typed tokens
- **Pratt parser** - operator-precedence parser for expressions (comparisons, AND/OR, NOT)
- **Supported statements**
  - SELECT ... FROM ... WHERE ... ORDER BY ...
  - CREATE TABLE ... (...)
  - CREATE INDEX ... ON ...
  - CREATE UNIQUE INDEX ... ON ...
- **Interactive REPL** - type SQL statements and see the parsed AST printed to the console

## Project Structure

```
sql_parser/
├── Cargo.toml
└── src/
    ├── main.rs        # REPL loop and AST pretty-printer
    ├── token.rs       # Token types and keyword lookup
    ├── tokenizer.rs   # Lexer / tokenizer implementation
    ├── statement.rs   # AST node definitions (Statement, Expr, ...)
    └── parser.rs      # Pratt parser
```

## Build

```bash
cargo build
```

## Run

```bash
cargo run
```

## Usage Examples

```sql
> SELECT id, name FROM users WHERE age > 18 ORDER BY name
Select {
  columns: [id, name],
  from: "users",
  where: Some(age > 18),
  orderby: [name]
}

> CREATE TABLE orders (id INT PRIMARY KEY, total FLOAT NOT NULL)
CreateTable {
  table_name: "orders",
  column_list: [
  TableColumn {
    column_name: "id",
    column_type: Int,
    constraints: [PrimaryKey]
  },
  TableColumn {
    column_name: "total",
    column_type: Float,
    constraints: [NotNull]
  }
  ]
}

> CREATE INDEX idx_name ON users (name)
CreateIndex {
  index_name: "idx_name",
  table_name: "users",
  columns: ["name"]
}
```

## License

MIT
