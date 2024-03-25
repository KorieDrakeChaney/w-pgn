use std::{iter::Peekable, str::Chars};

use crate::{token::Token, Nag};

enum State {
    Default,
    Symbol,
    String,
    Nag,
    CommentCurlyBracket,
    CommentSemicolon,
}

pub struct Tokenizer<'a> {
    input: Peekable<Chars<'a>>,

    current_state: State,

    temporary_buffer: Option<String>,

    current_input_character: Option<char>,

    reconsume: bool,

    pub tokens: Vec<Token>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut tokenizer = Tokenizer {
            input: input.chars().peekable(),
            current_state: State::Default,
            temporary_buffer: None,

            current_input_character: None,
            reconsume: false,

            tokens: Vec::new(),
        };

        tokenizer.tokenize();

        tokenizer
    }

    fn tokenize(&mut self) {
        while let Some(c) = self.consume() {
            match self.current_state {
                State::Default => match c {
                    '[' => {
                        self.tokens.push(Token::LeftBracket);
                    }
                    ';' => {
                        self.switch_to(State::CommentSemicolon);
                    }
                    '{' => {
                        self.switch_to(State::CommentCurlyBracket);
                    }
                    '(' => {
                        self.tokens.push(Token::LeftParenthesis);
                    }
                    'a'..='z' | 'A'..='Z' | '0'..='9' => {
                        self.append_to_temporary_buffer(c);
                        self.switch_to(State::Symbol);
                    }
                    '"' => {
                        self.switch_to(State::String);
                    }
                    '$' => {
                        self.switch_to(State::Nag);
                        self.append_to_temporary_buffer(c);
                    }
                    ']' => {
                        self.tokens.push(Token::RightBracket);
                    }
                    ')' => {
                        self.tokens.push(Token::RightParenthesis);
                    }
                    _ => {}
                },
                State::Symbol => match c {
                    'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '+' | '#' | '=' | ':' | '-' | '/' => {
                        self.append_to_temporary_buffer(c);
                    }
                    '.' => {
                        let symbol = self.get_temporary_buffer();
                        self.tokens.push(Token::Symbol(symbol));
                        self.switch_to(State::Default);
                    }
                    _ => {
                        let symbol = self.get_temporary_buffer();
                        match symbol.as_str() {
                            "0-1" | "1-0" | "1/2-1/2" | "*" => {
                                self.tokens.push(Token::Termination(symbol));
                                self.reconsume_in(State::Default);
                            }
                            _ => {
                                if symbol.len() > 0 {
                                    self.tokens.push(Token::Symbol(symbol));
                                    self.reconsume_in(State::Default);
                                }
                            }
                        }
                    }
                },
                State::String => match c {
                    '"' => {
                        let string = self.get_temporary_buffer();
                        self.tokens.push(Token::String(string));
                        self.switch_to(State::Default);
                    }
                    _ => {
                        self.append_to_temporary_buffer(c);
                    }
                },
                State::CommentCurlyBracket => match c {
                    '}' => {
                        let comment = self.get_temporary_buffer();
                        self.tokens.push(Token::Comment(comment));
                        self.switch_to(State::Default);
                    }
                    _ => {
                        self.append_to_temporary_buffer(c);
                    }
                },
                State::CommentSemicolon => match c {
                    '\n' => {
                        let comment = self.get_temporary_buffer();
                        self.tokens.push(Token::Comment(comment));
                        self.switch_to(State::Default);
                    }
                    _ => {
                        self.append_to_temporary_buffer(c);
                    }
                },
                State::Nag => {
                    if c.is_digit(10) {
                        self.append_to_temporary_buffer(c);
                    } else {
                        let nag = self.get_temporary_buffer();
                        self.tokens.push(Token::Nag(Nag::from(nag.as_str())));
                        self.reconsume_in(State::Default);
                    }
                }
            }
        }
        let temporary_buffer = self.get_temporary_buffer();
        match temporary_buffer.as_str() {
            "0-1" | "1-0" | "1/2-1/2" | "*" => {
                self.tokens.push(Token::Termination(temporary_buffer));
            }
            _ => {
                if temporary_buffer.len() > 0 {
                    self.tokens.push(Token::Symbol(temporary_buffer));
                }
            }
        }
    }

    fn append_to_temporary_buffer(&mut self, c: char) {
        if let Some(buffer) = &mut self.temporary_buffer {
            buffer.push(c);
        } else {
            self.temporary_buffer = Some(c.to_string());
        }
    }

    fn get_temporary_buffer(&mut self) -> String {
        match self.temporary_buffer.take() {
            Some(buffer) => buffer,
            None => String::new(),
        }
    }

    fn switch_to(&mut self, state: State) {
        self.current_state = state;
    }

    fn consume(&mut self) -> Option<char> {
        if self.reconsume {
            self.reconsume = false;
            return self.current_input_character;
        }

        self.current_input_character = self.input.next();

        self.current_input_character
    }

    fn reconsume(&mut self) {
        self.reconsume = true;
    }

    fn reconsume_in(&mut self, state: State) {
        self.reconsume();
        self.switch_to(state);
    }
}

impl IntoIterator for Tokenizer<'_> {
    type Item = Token;
    type IntoIter = std::vec::IntoIter<Token>;

    fn into_iter(self) -> Self::IntoIter {
        self.tokens.into_iter()
    }
}

#[cfg(test)]
mod tests {

    use std::io::Read;

    use super::*;

    #[test]
    fn test_tokenize_data() {
        let input = "[Event \"FIDE World Championship\"]";
        let tokenizer = Tokenizer::new(input);

        assert_eq!(
            tokenizer.tokens,
            vec![
                Token::LeftBracket,
                Token::Symbol("Event".to_string()),
                Token::String("FIDE World Championship".to_string()),
                Token::RightBracket,
            ]
        );
    }

    #[test]
    fn test_can_read_pgn() {
        let mut buffer = String::new();

        let mut file = std::fs::File::open("game.pgn").unwrap();

        file.read_to_string(&mut buffer).unwrap();

        let tokenizer = Tokenizer::new(&buffer);

        println!("{:?}", tokenizer.tokens);
    }

    #[test]
    fn test_can_read_string() {
        let input = r#"[Event "ch-UZB 1st League 2014"]
        [Site "Tashkent UZB"]
        [Date "2014.03.01"]
        [Round "1.5"]
        [White "Abdusattorov,Nodirbek"]
        [Black "Alikulov,Elbek"]
        [Result "1-0"]
        [WhiteElo "2024"]
        [BlackElo "2212"]
        [ECO "B28"]
        
        1.e4 c5 2.Nf3 a6 3.d3 g6 4.g3 Bg7 5.Bg2 b5 6.O-O Bb7 7.c3 e5 8.a3 Ne7 9.b4 d6
        10.Nbd2 O-O 11.Nb3 Nd7 12.Be3 Rc8 13.Rc1 h6 14.Nfd2 f5 15.f4 Kh7 16.Qe2 cxb4
        17.axb4 exf4 18.Bxf4 Rxc3 19.Rxc3 Bxc3 20.Bxd6 Qb6+ 21.Bc5 Nxc5 22.bxc5 Qe6
        23.d4 Rd8 24.Qd3 Bxd2 25.Nxd2 fxe4 26.Nxe4 Nf5 27.d5 Qe5 28.g4 Ne7 29.Rf7+ Kg8
        30.Qf1 Nxd5 31.Rxb7 Qd4+ 32.Kh1 Rf8 33.Qg1 Ne3 34.Re7 a5 35.c6 a4 36.Qxe3 Qxe3
        37.Nf6+ Rxf6 38.Rxe3 Rd6 39.h4 Rd1+ 40.Kh2 b4 41.c7  1-0
        "#;

        let tokenizer = Tokenizer::new(input);

        println!("{:?}", tokenizer.tokens);
    }
}
