#![forbid(unsafe_code)]

use std::any::Any;
////////////////////////////////////////////////////////////////////////////////

pub trait Agent : Any {
    fn make_move(&mut self) -> Move;
    fn as_any(&mut self) -> &mut dyn Any;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Move {
    None,
    Cooperate,
    Cheat,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RoundOutcome {
    BothCooperated,
    LeftCheated,
    RightCheated,
    BothCheated,
}

pub struct Game {
    left: Box<dyn Agent>,
    right: Box<dyn Agent>,
    left_score: i32,
    right_score: i32,
}

impl Game {
    pub fn new(left: Box<dyn Agent>, right: Box<dyn Agent>) -> Self {
        Self {
            left,
            right,
            left_score: 0,
            right_score: 0,
        }
    }   

    pub fn left_score(&self) -> i32 {
        self.left_score
    }

    pub fn right_score(&self) -> i32 {
        self.right_score
    }

    pub fn play_round(&mut self) -> RoundOutcome {
        match (self.left.make_move(), self.right.make_move()) {
            (Move::Cooperate, Move::Cooperate) => {
                Self::send_move(&mut self.left, Move::Cooperate);
                Self::send_move(&mut self.right, Move::Cooperate);

                self.left_score += 2;
                self.right_score += 2;
                RoundOutcome::BothCooperated
            },
            (Move::Cooperate, Move::Cheat) => {
                Self::send_move(&mut self.left, Move::Cheat);
                Self::send_move(&mut self.right, Move::Cooperate);

                self.left_score -= 1;
                self.right_score += 3;
                RoundOutcome::RightCheated
            },
            (Move::Cheat, Move::Cooperate) => {
                Self::send_move(&mut self.left, Move::Cooperate);
                Self::send_move(&mut self.right, Move::Cheat);

                self.left_score += 3;
                self.right_score -= 1;
                RoundOutcome::LeftCheated
            },
            (Move::Cheat, Move::Cheat) => {
                Self::send_move(&mut self.left, Move::Cheat);
                Self::send_move(&mut self.right, Move::Cheat);

                RoundOutcome::BothCheated
            },
            _ => unreachable!()
        }
    }

    fn send_move(receiver: &mut Box<dyn Agent>, mov: Move) {
        match receiver.as_any().downcast_mut::<GrudgerAgent>() {
            Some(grudger) => match mov {
                Move::Cheat => {
                    grudger.has_opponent_cheated = true;
                    return;
                },
                _ => ()
            }
            _ => (),
        };

        match receiver.as_any().downcast_mut::<CopycatAgent>() {
            Some(copycat) => {
                copycat.opponent_last_move = mov;
                return;
            },
            _ => (),
        };

        match receiver.as_any().downcast_mut::<DetectiveAgent>() {
            Some(detective) => {
                detective.opponent_last_move = mov;
                return;
            },
            _ => (),
        };
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct CheatingAgent {}

impl Agent for CheatingAgent {
    fn make_move(&mut self) -> Move {
        Move::Cheat
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct CooperatingAgent {}

impl Agent for CooperatingAgent {
    fn make_move(&mut self) -> Move {
        Move::Cooperate
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct GrudgerAgent {
    has_opponent_cheated: bool,
}

impl Default for GrudgerAgent {
    fn default() -> Self {
        Self {
            has_opponent_cheated: false,
        }
    }
}

impl Agent for GrudgerAgent {
    fn make_move(&mut self) -> Move {
        match self.has_opponent_cheated {
            true => Move::Cheat,
            false => Move::Cooperate,
        }
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct CopycatAgent {
    opponent_last_move: Move,
}

impl Default for CopycatAgent {
    fn default() -> Self {
        Self {
            opponent_last_move: Move::None,
        }
    }
}

impl Agent for CopycatAgent {
    fn make_move(&mut self) -> Move {
        match self.opponent_last_move {
            Move::Cooperate => Move::Cooperate,
            Move::Cheat => Move::Cheat,
            Move::None => Move::Cooperate
        }
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct DetectiveAgent {
    round_number: usize,
    has_opponent_cheated: bool,
    opponent_last_move: Move,
}

impl Default for DetectiveAgent {
    fn default() -> Self {
        Self {
            round_number: 0,
            has_opponent_cheated: false,
            opponent_last_move: Move::None,
        }
    }
}

impl Agent for DetectiveAgent {
    fn make_move(&mut self) -> Move {
       let mov =  match self.round_number {
                            0     => Move::Cooperate,
                            1     => Move::Cheat,
                            2 | 3 => Move::Cooperate,
                            _     => match self.has_opponent_cheated {
                                        true => self.opponent_last_move,
                                        false => Move::Cheat
                                    },
                                
                        };
        if self.opponent_last_move == Move::Cheat {
            self.has_opponent_cheated = true;
        }
        self.round_number += 1;
        mov

    }  

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}
