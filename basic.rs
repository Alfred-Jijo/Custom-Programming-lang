//----------------------------------//
//              Constants           //
//----------------------------------//

const DIGITS: &str = "0123456789";

//----------------------------------//
//              Errors              //
//----------------------------------//

#[derive(Debug)]
struct Error {
    pos_start: Position,
    pos_end: Position,
    error_name: String,
    details: String,
}

impl Error {
    fn new(pos_start: Position, pos_end: Position, error_name: &str, details: &str) -> Self {
        Error {
            pos_start,
            pos_end,
            error_name: String::from(error_name),
            details: String::from(details),
        }
    }

    fn as_string(&self) -> String {
        format!(
            "{}: {}\nFile {}, line {}",
            self.error_name,
            self.details,
            self.pos_start.fn_name,
            self.pos_start.ln + 1
        )
    }
}

#[derive(Debug)]
struct Position {
    idx: usize,
    ln: usize,
    col: usize,
    fn_name: String,
    ftxt: String,
}

impl Position {
    fn new(idx: usize, ln: usize, col: usize, fn_name: &str, ftxt: &str) -> Self {
        Position {
            idx,
            ln,
            col,
            fn_name: String::from(fn_name),
            ftxt: String::from(ftxt),
        }
    }

    fn advance(&mut self, current_char: Option<char>) {
        self.idx += 1;
        self.col += 1;

        if let Some('\n') = current_char {
            self.ln += 1;
            self.col = 0;
        }
    }

    fn copy(&self) -> Self {
        Position {
            idx: self.idx,
            ln: self.ln,
            col: self.col,
            fn_name: self.fn_name.clone(),
            ftxt: self.ftxt.clone(),
        }
    }
}

//----------------------------------//
//              Tokens              //
//----------------------------------//

#[derive(Debug)]
enum TokenType {
    INT,
    FLOAT,
    PLUS,
    MINUS,
    MUL,
    DIV,
    LPAREN,
    RPAREN,
}

#[derive(Debug)]
struct Token {
    type_: TokenType,
    value: Option<String>,
}

impl Token {
    fn new(type_: TokenType, value: Option<String>) -> Self {
        Token { type_, value }
    }
}

//----------------------------------//
//              Lexer               //
//----------------------------------//

struct Lexer {
    fn_name: String,
    text: String,
    pos: Position,
    current_char: Option<char>,
}

impl Lexer {
    fn new(fn_name: &str, text: &str) -> Self {
        let mut lexer = Lexer {
            fn_name: String::from(fn_name),
            text: String::from(text),
            pos: Position::new(0, 0, -1, fn_name, text),
            current_char: None,
        };
        lexer.advance();
        lexer
    }

    fn advance(&mut self) {
        self.pos.advance(self.current_char);
        self.current_char = self.text.chars().nth(self.pos.idx);
    }

    fn make_tokens(&mut self) -> (Vec<Token>, Option<Error>) {
        let mut tokens = Vec::new();

        while let Some(current_char) = self.current_char {
            if current_char.is_whitespace() {
                self.advance();
                continue;
            }

            if DIGITS.contains(current_char) {
                match self.make_number() {
                    Ok(token) => tokens.push(token),
                    Err(error) => return (Vec::new(), Some(error)),
                }
            } else {
                let pos_start = self.pos.copy();
                let char_str = current_char.to_string();
                self.advance();
                return (
                    Vec::new(),
                    Some(Error::new(
                        pos_start,
                        self.pos.copy(),
                        "Illegal Character",
                        &format!("'{}'", char_str),
                    )),
                );
            }

            self.advance();
        }

        (tokens, None)
    }

    fn make_number(&mut self) -> Result<Token, Error> {
        let mut num_str = String::new();
        let mut dot_count = 0;

        while let Some(current_char) = self.current_char {
            if DIGITS.contains(current_char) {
                if current_char == '.' {
                    if dot_count == 1 {
                        break;
                    }
                    dot_count += 1;
                    num_str.push('.');
                } else {
                    num_str.push(current_char);
                }
            } else {
                break;
            }
            self.advance();
        }

        if dot_count == 0 {
            Ok(Token::new(TokenType::INT, Some(num_str)))
        } else {
            Ok(Token::new(TokenType::FLOAT, Some(num_str)))
        }
    }
}





fn run(fn_name: &str, text: &str) -> (Vec<Token>, Option<Error>) {
    let mut lexer = Lexer::new(fn_name, text);
    lexer.make_tokens()
}

fn main() {
    let fn_name = "example";
    let text = "3.14 + 2 * (6 - 4.5)";
    let (tokens, error) = run(fn_name, text);

    println!("Tokens: {:?}", tokens);
    if let Some(error) = error {
        println!("{}", error.as_string());
    }
}
