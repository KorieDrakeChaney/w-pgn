use std::cell::RefCell;
use std::rc::{Rc, Weak};

pub type ChessMoveNode = Rc<RefCell<ChessMove>>;
pub type ChessMoveTree = Vec<ChessMoveNode>;

#[derive(Debug)]
pub struct ChessMove {
    pub san: String,
    pub turn: usize,
    pub comment: Option<String>,
    pub variations: ChessMoveTree,
    pub parent: Option<Weak<RefCell<ChessMove>>>,
}

impl ChessMove {
    pub fn new(san: &str) -> ChessMoveNode {
        Rc::new(RefCell::new(ChessMove {
            san: san.to_string(),
            turn: 1,
            comment: None,
            variations: Vec::new(),
            parent: None,
        }))
    }

    pub fn add_variation(&mut self, variation: ChessMoveNode) {
        variation.borrow_mut().turn = (self.variations.len() + 1) / 2 + self.turn;
        self.variations.push(variation);
    }

    pub fn add_comment(&mut self, comment: &str) {
        self.comment = Some(comment.to_string());
    }

    pub fn get_piece_at_turn(&self, turn: usize) -> Option<ChessMoveNode> {
        for variation in &self.variations {
            if variation.borrow().turn == turn {
                return Some(variation.clone());
            }
        }

        None
    }

    pub fn print_tree(&self, depth: usize) {
        let mut indent = String::new();
        for _ in 0..depth {
            indent.push_str("  ");
        }

        println!("{} {}. {}", indent, self.turn, self.san);

        if let Some(comment) = &self.comment {
            println!("{}  Comment: {}", indent, comment);
        }

        for variation in &self.variations {
            variation.borrow().print_tree(depth + 1);
        }
    }

    pub fn get_moves(&self) -> Vec<String> {
        let mut moves = vec![self.san.clone()];
        for variation in &self.variations {
            moves.push(variation.borrow().san.clone());
        }

        moves
    }
}
