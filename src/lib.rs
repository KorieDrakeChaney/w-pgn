mod chess_move;
mod nag;
mod token;
mod tokenizer;
use std::{collections::HashMap, rc::Rc};

use chess_move::{ChessMove, ChessMoveNode};
pub use nag::Nag;
use token::Token;
use tokenizer::Tokenizer;

#[derive(Debug)]
pub struct PGN {
    // Seven Tag Roster
    pub event: String,
    pub site: String,
    pub date: String,
    pub round: String,
    pub white: String,
    pub black: String,
    pub result: String,

    pub tags: HashMap<String, String>,

    pub moves: Option<ChessMoveNode>,
}

impl PGN {
    pub fn parse(pgn: &str) -> std::io::Result<Self> {
        let tokens = Tokenizer::new(pgn).tokens;

        PGN::parse_tokens(&tokens)
    }

    pub fn parse_tokens(tokens: &Vec<Token>) -> std::io::Result<Self> {
        let mut tags = HashMap::new();
        let mut event = None;
        let mut site = None;
        let mut date = None;
        let mut round = None;
        let mut white = None;
        let mut black = None;
        let mut result = None;

        let mut index = 0;

        let mut variation_depth = Vec::new();

        let mut moves = None;

        while let Some(token) = tokens.get(index) {
            match token {
                Token::LeftBracket => {
                    if let Some(Token::Symbol(symbol)) = tokens.get(index + 1) {
                        if let Some(Token::String(value)) = tokens.get(index + 2) {
                            match symbol.as_str() {
                                "Event" => {
                                    event = Some(value.clone());
                                }
                                "Site" => {
                                    site = Some(value.clone());
                                }
                                "Date" => {
                                    date = Some(value.clone());
                                }
                                "Round" => {
                                    round = Some(value.clone());
                                }
                                "White" => {
                                    white = Some(value.clone());
                                }
                                "Black" => {
                                    black = Some(value.clone());
                                }
                                "Result" => {
                                    result = Some(value.clone());
                                }
                                _ => {
                                    tags.insert(symbol.clone(), value.clone());
                                }
                            }
                        } else {
                            return Err(std::io::Error::new(
                                std::io::ErrorKind::InvalidData,
                                "Needed a string after tag symbol",
                            ));
                        }

                        if let Some(Token::RightBracket) = tokens.get(index + 3) {
                            index += 4;
                        } else {
                            return Err(std::io::Error::new(
                                std::io::ErrorKind::InvalidData,
                                "Needed a RightBracket after tag value",
                            ));
                        }
                    } else {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            "Needed a symbol after LeftBracket",
                        ));
                    }
                }
                Token::RightBracket => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "Unexpected RightBracket",
                    ));
                }

                Token::Symbol(m) => {
                    if !is_valid_san(m) {
                        index += 1;
                    } else {
                        if let Some(ref mut current_move) = moves {
                            let variation = ChessMove::new(m);
                            if variation_depth.len() == 0 {
                                variation.borrow_mut().parent = Some(Rc::downgrade(current_move));
                                current_move.borrow_mut().add_variation(variation.clone());
                            } else if let Some(variation_turn) = variation_depth.last() {
                                if let Some(parent) =
                                    current_move.borrow().get_piece_at_turn(*variation_turn)
                                {
                                    variation.borrow_mut().parent = Some(Rc::downgrade(&parent));
                                    parent.borrow_mut().add_variation(variation.clone());
                                }
                            }
                        } else {
                            moves = Some(ChessMove::new(m));
                        }
                        index += 1;
                    }
                }

                Token::LeftParenthesis => {
                    if let Some(Token::Symbol(symbol)) = tokens.get(index + 1) {
                        if let Some(turn) = symbol.parse::<usize>().ok() {
                            variation_depth.push(turn);
                        } else {
                            return Err(std::io::Error::new(
                                std::io::ErrorKind::InvalidData,
                                "Expected a number after LeftParenthesis",
                            ));
                        }
                    }
                    index += 2;
                }

                Token::RightParenthesis => {
                    if variation_depth.len() > 0 {
                        variation_depth.pop();
                    } else {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            "Unexpected RightParenthesis",
                        ));
                    }
                    index += 1;
                }

                Token::String(s) => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!("Unexpected string: {}", s),
                    ));
                }

                Token::Termination(s) => {
                    tags.insert("Termination".to_string(), s.clone());
                    break;
                }

                _ => {
                    index += 1;
                }
            }
        }

        if let (
            Some(event),
            Some(site),
            Some(date),
            Some(round),
            Some(white),
            Some(black),
            Some(result),
        ) = (event, site, date, round, white, black, result)
        {
            Ok(PGN {
                event,
                site,
                date,
                round,
                white,
                black,
                result,
                tags,
                moves,
            })
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Missing required tag(s)",
            ))
        }
    }

    pub fn parse_multiple(pgns: &str) -> std::io::Result<Vec<Self>> {
        let mut pgn_vec = Vec::new();

        let all_tokens = Tokenizer::new(pgns).tokens;

        let mut parseable_tokens = Vec::new();

        for token in all_tokens {
            match token {
                Token::Termination(_) => {
                    if let Ok(pgn) = PGN::parse_tokens(&parseable_tokens) {
                        pgn_vec.push(pgn);
                    }
                    parseable_tokens.clear();
                }
                _ => {
                    parseable_tokens.push(token);
                }
            }
        }

        Ok(pgn_vec)
    }

    pub fn moves(&self) -> Vec<String> {
        if let Some(moves) = &self.moves {
            moves.borrow().get_moves()
        } else {
            Vec::new()
        }
    }
}

impl std::fmt::Display for PGN {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[Event \"{}\"]\n", self.event)?;
        write!(f, "[Site \"{}\"]\n", self.site)?;
        write!(f, "[Date \"{}\"]\n", self.date)?;
        write!(f, "[Round \"{}\"]\n", self.round)?;
        write!(f, "[White \"{}\"]\n", self.white)?;
        write!(f, "[Black \"{}\"]\n", self.black)?;
        write!(f, "[Result \"{}\"]\n", self.result)?;

        for (key, value) in &self.tags {
            write!(f, "[{} \"{}\"]\n", key, value)?;
        }

        if let Some(moves) = &self.moves {
            let mut index = 0;
            let mut turn = 1;
            let moves = moves.borrow().get_moves();

            while index < moves.len() {
                if index % 2 == 0 {
                    write!(f, "{}.", turn)?;
                    turn += 1;
                }

                write!(f, "{} ", moves[index])?;

                index += 1;
            }
        }

        if let Some(termination) = &self.tags.get(&"Termination".to_string()) {
            write!(f, "{}", termination)?;
        }

        Ok(())
    }
}

fn is_valid_san(san: &str) -> bool {
    let stripped_san: String = san
        .chars()
        .filter(|c| match c {
            '+' | '#' | '=' | '!' | '?' | 'x' => false,
            _ => true,
        })
        .collect();

    if stripped_san == "O-O" || stripped_san == "O-O-O" {
        return true;
    }

    if stripped_san.len() < 2 || stripped_san.len() > 4 {
        return false;
    }

    let mut chars = stripped_san.chars().peekable();

    // Piece check
    if let Some(&first_char) = chars.peek() {
        match first_char {
            'a'..='h' => {}
            'K' | 'Q' | 'R' | 'B' | 'N' => {
                chars.next();
            }
            _ => return false,
        }
    }

    for char in chars {
        match char {
            'a'..='h' | '1'..='8' | 'x' => (),
            _ => return false,
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use std::io::{Read, Write};

    use super::*;

    #[test]
    fn test_parse() {
        let pgn_input = r#"[Event "F/S Return Match"]
        [Site "Belgrade, Serbia JUG"]
        [Date "1992.11.04"]
        [Round "29"]
        [White "Fischer, Robert J."]
        [Black "Spassky, Boris V."]
        [Result "1/2-1/2"]

        1. e4 e5 2. Nf3 Nc6 3. Bb5 {This opening is called the Ruy Lopez.} 3... a6
        4. Ba4 Nf6 5. O-O Be7 6. Re1 b5 7. Bb3 d6 8. c3 O-O 9. h3 Nb8 10. d4 Nbd7
        11. c4 c6 12. cxb5 axb5 13. Nc3 Bb7 14. Bg5 b4 15. Nb1 h6 16. Bh4 c5 17. dxe5
        Nxe4 18. Bxe7 Qxe7 19. exd6 Qf6 20. Nbd2 Nxd6 21. Nc4 Nxc4 22. Bxc4 Nb6
        23. Ne5 Rae8 24. Bxf7+ Rxf7 25. Nxf7 Rxe1+ 26. Qxe1 Kxf7 27. Qe3 Qg5 28. Qxg5
        hxg5 29. b3 Ke6 30. a3 Kd6 31. axb4 cxb4 32. Ra5 Nd5 33. f3 Bc8 34. Kf2 Bf5
        35. Ra7 g6 36. Ra6+ Kc5 37. Ke1 Nf4 38. g3 Nxh3 39. Kd2 Kb5 40. Rd6 Kc5 41. Ra6
        Nf2 42. g4 Bd3 43. Re6 1/2-1/2
        "#;

        let pgn = PGN::parse(pgn_input).unwrap();

        assert_eq!(pgn.event, "F/S Return Match");
        assert_eq!(pgn.site, "Belgrade, Serbia JUG");
        assert_eq!(pgn.date, "1992.11.04");
        assert_eq!(pgn.round, "29");
        assert_eq!(pgn.white, "Fischer, Robert J.");
        assert_eq!(pgn.black, "Spassky, Boris V.");
        assert_eq!(pgn.result, "1/2-1/2");

        if let Some(moves) = pgn.moves {
            moves.borrow().print_tree(0);
        }
    }

    #[test]
    fn my_game() {
        let mut buffer = String::new();

        let mut file = std::fs::File::open("game.pgn").unwrap();

        file.read_to_string(&mut buffer).unwrap();

        let pgn = PGN::parse(&buffer).unwrap();

        if let Some(moves) = pgn.moves {
            moves.borrow().print_tree(0);
        }
    }

    #[test]
    fn multiple_games() {
        let mut buffer = String::new();

        let mut file = std::fs::File::open("Abdusattorov.pgn").unwrap();

        file.read_to_string(&mut buffer).unwrap();

        let pgns = PGN::parse_multiple(&buffer).unwrap();

        let mut buffer = String::new();

        for pgn in pgns {
            buffer += &format!("{}\n\n", pgn);
        }

        let mut file = std::fs::File::create("output.pgn").unwrap();

        file.write_all(buffer.as_bytes()).unwrap();
    }
}
