pub struct Position {
    pub line: usize,
    pub column: usize,
}

pub struct Token<TokenKind> {
    pub token_type: TokenKind,
    pub value: String,
    pub position: Position,
}

pub type TokenOptionsCallback<TK> = fn(char, Position, String) -> (Token<TK>, usize);

pub type TokenOptions<'a, TK> = Vec<(&'a str, TokenOptionsCallback<TK>)>;

pub fn tokenize<TK>(input: String, options: TokenOptions<TK>) -> Vec<Token<TK>> {
    let lines = input.lines();
    let mut tokens = Vec::new();
    for (line_number, line) in lines.enumerate() {
        let mut column = 0;
        for c in line.chars() {
            let mut token: Option<Token<TK>> = None;
            for (ch, option) in &options {
                let chars = *ch;
                if crate::_is_valid_char::is_valid_char(chars, c) == false {
                    continue;
                }
                let (t, consumed) = option(c, Position { line: line_number, column }, line.to_string());
                token = Some(t);
                column += consumed;
                break;
            }
            if let Some(token) = token {
                tokens.push(token);
            }
            column += 1;
        }
    }
    tokens
}
