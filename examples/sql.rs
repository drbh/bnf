const SQL_BNF: &str = r#"
<view_definition> ::= "CREATE VIEW" <ws> <identifier> <ws> "AS" <ws> <select_statement>

<select_statement> ::= "SELECT" <ws> <expr> <ws> "FROM" <ws> <expr>

<expr> ::= <number>
	| <identifier>
    | <identifier> <ws> "AS" <ws> <identifier>
    | <identifier> "." <identifier>

<ws> ::= " " | " " <ws>

<number> ::= "0" | "1" | "2" | "3" | "4" | "5"
            | "6" | "7" | "8" | "9"

<identifier> ::= <letter> | <letter> <identifier_rest>
<identifier_rest> ::= <letter> | <number> | "_" | <letter> <identifier_rest> | <number> <identifier_rest> | "_" <identifier_rest>

<letter> ::= "A" | "B" | "C" | "D" | "E" | "F"
            | "G" | "H" | "I" | "J" | "K" | "L"
            | "M" | "N" | "O" | "P" | "Q" | "R"
            | "S" | "T" | "U" | "V" | "W" | "X"
            | "Y" | "Z" | "a" | "b" | "c" | "d"
            | "e" | "f" | "g" | "h" | "i" | "j"
            | "k" | "l" | "m" | "n" | "o" | "p"
            | "q" | "r" | "s" | "t" | "u" | "v"
            | "w" | "x" | "y" | "z"
"#;

fn main() {
    let sql_grammar: bnf::Grammar = SQL_BNF.parse().unwrap();

    let input = "CREATE VIEW employed_names AS SELECT e.first_name FROM Employees AS e";

    for parse_tree in sql_grammar.parse_input(input) {
        dbg!(&parse_tree);
        // println!("{}", parse_tree.mermaid());
    }
}
