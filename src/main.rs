//declare modules for project 
pub mod statement;
pub mod token;
pub mod tokenizer;
pub mod parser;

use std::io::{self, Write};
use tokenizer::Tokenizer;
use parser::Parser;
use statement::Statement;


fn format_statement(statement: &Statement) -> String { //formats a parsed Statement into a multi-line string for console output
    match statement {
        Statement::Select { columns, from, r#where, orderby } => {
            let columns_str = columns.iter()
                .map(|col| format!("{}", col))
                .collect::<Vec<_>>()
                .join(", ");
            let where_str = match r#where {
                Some(expr) => format!("Some({})", expr),
                None => "None".to_string(),
            };
            let orderby_str = orderby.iter()
                .map(|ord| format!("{}", ord))
                .collect::<Vec<_>>()
                .join(", ");
            format!(
                "Select {{\n  columns: [{}],\n  from: \"{}\",\n  where: {},\n  orderby: [{}]\n}}",
                columns_str, from, where_str, orderby_str
            )
        }
        Statement::CreateTable { table_name, column_list } => {
            let columns_str = column_list.iter()
                .map(|col| {
                    let constraints = col.constraints.iter()
                        .map(|c| format!("{:?}", c))
                        .collect::<Vec<_>>()
                        .join(", ");
                    format!(
                        "TableColumn {{\n    column_name: \"{}\",\n    column_type: {:?},\n    constraints: [{}]\n  }}",
                        col.column_name, col.column_type, constraints
                    )
                })
                .collect::<Vec<_>>()
                .join(",\n  ");
            format!(
                "CreateTable {{\n  table_name: \"{}\",\n  column_list: [\n  {}\n  ]\n}}",
                table_name, columns_str
            )
        }
        Statement::CreateIndex { index_name, table_name, columns } => {
            let columns_str = columns.iter()
                .map(|col| format!("\"{}\"", col))
                .collect::<Vec<_>>()
                .join(", ");
            format!(
                "CreateIndex {{\n  index_name: \"{}\",\n  table_name: \"{}\",\n  columns: [{}]\n}}",
                index_name, table_name, columns_str
            )
        }
        Statement::CreateUniqueIndex { index_name, table_name, columns } => {
            let columns_str = columns.iter()
                .map(|col| format!("\"{}\"", col))
                .collect::<Vec<_>>()
                .join(", ");
            format!(
                "CreateUniqueIndex {{\n  index_name: \"{}\",\n  table_name: \"{}\",\n  columns: [{}]\n}}",
                index_name, table_name, columns_str
            )
        }
    }
}

fn main() { //main loop - reads SQL input, tokenizes, parses and prints formatted output
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Error reading input");
            continue;
        }
        let tokenizer = Tokenizer::new(&input);
        match Parser::new(tokenizer) {
            Ok(mut parser) => match parser.parse() {
                Ok(statement) => println!("{}", format_statement(&statement)),
                Err(e) => println!("Error: {}", e),
            },
            Err(e) => println!("Error: {}", e),
        }
    }
}
