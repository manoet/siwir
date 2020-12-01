// Copyright (c) 2019 Marco Giglio
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use crate::basic_parser::State;
use crate::basic_parser::Transition;

// name     := letter [alphanum_str]
// dotname  := (name '.')* name
// var      := ['$'] dotname
// args     := (id ',')* id
// fcall    := ['$'] dotname '(' args ')'
// value    := number
// number   := natural
// natural  := '0' | ('1' | '2' | ... | '9') digit*
// id       := const | fcall | var
// factor   := '(' expr ')' | id
// term     := factor (('*' | '/' | '%') factor)*
// expr     := term (('+' | '-') term)*
//
// cond         := '!' logic_factor | (expr ('==' | '!=' | '<' | '<=' | '>' | '>=') expr)
// logic_factor := '(' logic_expr ')' | cond
// logic_term   := logic_factor ('||' logic_factor)*
// logix_expr   := logic_term ('&&' logic_term)*

pub struct Parser {
}

impl Parser {
    pub fn new() -> Parser {
        Parser {}
    }

    pub fn parse(&self, s: &str) -> Option<bool> {
        None
    }

    fn ws(&self, state: &State) -> State {
        any!(alt!(chr!(' '), chr!('\t'), chr!('\n'), chr!('\r')))
            .matches(state)
            .unwrap()
    }

    fn natural(&self, state: &State) -> Option<State> {
        let trimmed = self.ws(&state);
        let zero = chr!('0').matches(&trimmed);
        if zero.is_some() {
            let next_char = zero.as_ref().unwrap().peek();
            if next_char.is_digit(10) { return None; }
            return zero;
        }
        seq!(chr!('1', '9'), rep!(digit!(), '*'))
            .matches(&trimmed)
    }

    fn number(&self, state: &State) -> Option<State> {
        self.natural(&state)
    }

    fn value(&self, state: &State) -> Option<State> {
        self.number(&state)
    }

    fn name(&self, state: &State) -> Option<State> {
        let trimmed = self.ws(&state);
        match letter!().matches(&trimmed) {
            Some(s) => opt!(alphanum_str!()).matches(&s),
            None => None,
        }
    }

    fn dotted_name(&self, state: &State) -> Option<State> {
        let mut curr = state.clone();
        loop {
            let name = self.name(&curr);
            if name.is_none() { return None; }
            let dot = chr!('.').matches(&name.as_ref().unwrap());
            if dot.is_none() { return name; }
            curr = dot.unwrap();
        }
    }

    fn var(&self, state: &State) -> Option<State> {
        let trimmed = self.ws(&state);
        let dollar = opt!(chr!('$')).matches(&trimmed);
        self.dotted_name(&dollar.unwrap())
    }

    fn args(&self, state: &State) -> Option<State> {
        let mut i = 0;
        let mut curr = state.clone();
        loop {
            let arg = self.id(&curr);
            if arg.is_none() {
                return match i {
                    0 => Some(curr),
                    _ => None,
                }
            }
            let trimmed = self.ws(&arg.as_ref().unwrap());
            let comma = chr!(',').matches(&trimmed);
            if comma.is_none() {
                return arg;
            }
            curr = comma.unwrap();
            i += 1;
        }
    }

    fn fcall(&self, state: &State) -> Option<State> {
        let trimmed = self.ws(&state);
        let dollar = opt!(chr!('$')).matches(&trimmed);
        let fn_name = self.dotted_name(&dollar.unwrap());
        if fn_name.is_none() { return None; }
        let lbrace = chr!('(').matches(&fn_name.unwrap());
        if lbrace.is_none() { return None; }
        let args = self.args(&lbrace.unwrap());
        if args.is_none() { return None; }
        chr!(')').matches(&args.unwrap())
    }

    fn id(&self, state:&State) -> Option<State> {
        let mut ret = self.fcall(&state);
        if ret.is_some() { return ret; }
        ret = self.value(&state);
        if ret.is_some() { return ret; }
        self.var(&state)
    }

    fn factor(&self, state: &State) -> Option<State> {
        let mut trimmed = self.ws(&state);
        let id = self.id(&trimmed);
        if id.is_some() { return id; }
        let lbrace = chr!('(').matches(&trimmed);
        if lbrace.is_none() {return None;}
        let expr = self.expr(&lbrace.unwrap());
        if expr.is_none() { return None; }
        trimmed = self.ws(&expr.unwrap());
        chr!(')').matches(&trimmed)
    }

    fn term_rhs(&self, state: &State) -> Option<State> {
        let trimmed = self.ws(&state);
        let op = alt!(chr!('*'), chr!('/'), chr!('%')).matches(&trimmed);
        if op.is_none() { return None; }
        self.factor(&op.unwrap())
    }

    fn term(&self, state: &State) -> Option<State> {
        let mut ret = self.factor(&state);
        if ret.is_none() {
            return None;
        }
        loop {
            let curr = self.term_rhs(&ret.as_ref().unwrap());
            if curr.is_none() { return ret; }
            ret = curr;
        }
    }

    fn expr_rhs(&self, state: &State) -> Option<State> {
        let trimmed = self.ws(&state);
        let op = alt!(chr!('+'), chr!('-')).matches(&trimmed);
        if op.is_none() { return None; }
        self.term(&op.unwrap())
    }

    fn expr(&self, state: &State) -> Option<State> {
        let mut ret = self.term(&state);
        if ret.is_none() { return None; }
        loop {
            let curr = self.expr_rhs(&ret.as_ref().unwrap());
            if curr.is_none() { return ret; }
            ret = curr;
        }
    }

    fn condition(&self, state: &State) -> Option<State> {
        let mut trimmed = self.ws(&state);
        let not = chr!('!').matches(&trimmed);
        if not.is_some() { return self.logic_factor(&not.unwrap()); }
        let expr = self.expr(&trimmed);
        if expr.is_none() { return None; }
        trimmed = self.ws(&expr.unwrap());
        let op = alt!(seq!(chr!('='), chr!('=')),
                      seq!(chr!('!'), chr!('=')),
                      seq!(chr!('<'), chr!('=')),
                      seq!(chr!('>'), chr!('=')),
                      chr!('<'),
                      chr!('>')).matches(&trimmed);
        if op.is_none() { return None; }
        self.expr(&op.unwrap())
    }

    fn logic_factor(&self, state: &State) -> Option<State> {
        let mut trimmed = self.ws(&state);
        let cond = self.condition(&trimmed);
        if cond.is_some() { 
            return cond;
        }
        let lbrace = chr!('(').matches(&trimmed);
        if lbrace.is_none() {return None;}
        let expr = self.logic_expr(&lbrace.unwrap());
        if expr.is_none() { return None; }
        trimmed = self.ws(&expr.unwrap());
        chr!(')').matches(&trimmed)
    }

    fn logic_term_rhs(&self, state: &State) -> Option<State> {
        let trimmed = self.ws(&state);
        let op = seq!(chr!('&'), chr!('&')).matches(&trimmed);
        if op.is_none() { return None; }
        self.logic_factor(&op.unwrap())
    }

    fn logic_term(&self, state: &State) -> Option<State> {
        let mut ret = self.logic_factor(&state);
        if ret.is_none() { return None; }
        loop {
            let curr = self.logic_term_rhs(&ret.as_ref().unwrap());
            if curr.is_none() { return ret; }
            ret = curr;
        }
    }

    fn logic_expr_rhs(&self, state: &State) -> Option<State> {
        let trimmed = self.ws(&state);
        let op = seq!(chr!('|'), chr!('|')).matches(&trimmed);
        if op.is_none() { return None; }
        self.logic_term(&op.unwrap())
    }

    fn logic_expr(&self, state: &State) -> Option<State> {
        let mut ret = self.logic_term(&state);
        if ret.is_none() { return None; }
        loop {
            let curr = self.logic_expr_rhs(&ret.as_ref().unwrap());
            if curr.is_none() { return ret; }
            ret = curr;
        }
    }
}

mod ts_parser {

use crate::parser::Parser;
use crate::basic_parser::State;

macro_rules! assert_complete {
    ($match_result:expr) => {
        assert!($match_result.is_some());
        assert!($match_result.unwrap().complete());
    };
}

macro_rules! assert_next {
    ($match_result:expr, $next:expr) => {
        assert!($match_result.is_some());
        let ret = $match_result.unwrap();
        assert!(!ret.complete());
        assert_eq!(ret.peek(), $next);
    };
}

macro_rules! assert_none {
    ($match_result:expr) => {
        assert!($match_result.is_none());
    };
}

#[test]
fn parse_number() {
    let p = Parser::new();
    assert_complete!(p.number(&State::from_string(" 1")));
    assert_complete!(p.number(&State::from_string(" 0")));
    assert_complete!(p.number(&State::from_string(" 12034")));
    assert_none!(p.number(&State::from_string("01234")));
}

#[test]
fn parse_name() {
    let p = Parser::new();
    assert_complete!(p.name(&State::from_string("n")));
    assert_complete!(p.name(&State::from_string("name")));
    assert_complete!(p.name(&State::from_string("complex_name0")));
    assert_none!(p.name(&State::from_string("_name")));
    assert_none!(p.name(&State::from_string("0name")));
}

#[test]
fn parse_dotted_name() {
    let p = Parser::new();
    assert_complete!(p.dotted_name(&State::from_string("dotted.name.var")));
    assert_none!(p.dotted_name(&State::from_string(".invalid.name")));
}

#[test]
fn parse_var() {
    let p = Parser::new();
    assert_complete!(p.var(&State::from_string("var")));
    assert_complete!(p.var(&State::from_string("$dotted.var")));
}

#[test]
fn parse_fcall() {
    let p = Parser::new();
    assert_complete!(p.fcall(&State::from_string("fn(arg)")));
    assert_complete!(p.fcall(&State::from_string("fn()")));
    assert_complete!(p.fcall(&State::from_string("$fn(arg0, arg1)")));
    assert_complete!(p.fcall(&State::from_string("$fn($arg0(z), 21)")));
}

#[test]
fn id() {
    let p = Parser::new();
    assert_complete!(p.id(&State::from_string("var")));
    assert_complete!(p.id(&State::from_string("$var")));
    assert_complete!(p.id(&State::from_string("fn(arg)")));
    assert_complete!(p.id(&State::from_string("$fn(arg)")));
    assert_complete!(p.id(&State::from_string("0")));
}

#[test]
fn parse_factor() {
    let p = Parser::new();
    assert_complete!(p.factor(&State::from_string("42")));
    assert_complete!(p.factor(&State::from_string(" ( 42+21 )")));
    assert_complete!(p.factor(&State::from_string(" ( $var )")));
}

#[test]
fn parse_term() {
    let p = Parser::new();
    assert_complete!(p.term(&State::from_string("42 * 21")));
    assert_complete!(p.term(&State::from_string("42 / 21")));
    assert_complete!(p.term(&State::from_string("42 % 21")));
    assert_complete!(p.term(&State::from_string("42")));
    assert_complete!(p.term(&State::from_string("(1 + 2) * (3 + 4 + 5) / 6")));
}

#[test]
fn parse_expr() {
    let p = Parser::new();
    assert_complete!(p.expr(&State::from_string("31")));
    assert_complete!(p.expr(&State::from_string("31 * 91")));
    assert_complete!(p.expr(&State::from_string("31 * 91 + 21")));
    assert_complete!(p.expr(&State::from_string("31 * 91 + 21 - 51")));
    assert_complete!(p.expr(&State::from_string("31 * 91 + 21 - 51/41 % 21")));
    assert_complete!(p.expr(&State::from_string("1 + 2  +3")));
}

#[test]
fn parse_condition() {
    let p = Parser::new();
    assert_complete!(p.condition(&State::from_string("2 + 42 < 15 - 3")));
    assert_complete!(p.condition(&State::from_string("2 + 42 <= 15 - 3")));
    assert_complete!(p.condition(&State::from_string("2 + 42 > 15 - 3")));
    assert_complete!(p.condition(&State::from_string("2 + 42 >= 15 - 3")));
    assert_complete!(p.condition(&State::from_string("2 + 42 == 15 - 3")));
    assert_complete!(p.condition(&State::from_string("2 + 42 != 15 - 3")));
    assert_complete!(p.condition(&State::from_string("!(2 > 3)")));
}

#[test]
fn parse_logic_term() {
    let p = Parser::new();
    assert_complete!(p.logic_term(&State::from_string("2 < 3 && 5 > 4")));
}

#[test]
fn parse_logic_expr() {
    let p = Parser::new();
    assert_complete!(p.logic_expr(&State::from_string("2 < 3 || 5 > 4")));
}

}
