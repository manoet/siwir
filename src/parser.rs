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

use crate::ast;
use crate::basic_parser::State;
use crate::basic_parser::Transition;

// name     := letter [alphanum_str]
// dotname  := (name '.')* name
// var      := ['$'] dotname
// args     := (expr (',' expr)*)?
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
// logic_expr   := logic_term ('&&' logic_term)*

struct ParserState {
    pub state: State,
    pub node: Box<dyn ast::AstNode>,
}

impl ParserState {
    fn new<T: ast::StringInitialized>(s: State) -> ParserState {
        ParserState {
            node: T::from_str(s.matched()),
            state: s,
        }
    }

    fn complete(&self) -> bool {
        self.state.complete()
    }

    fn update(&mut self, other: &State) {
        self.state.update(other);
    }

    fn append_child(&mut self, child: ParserState) {
        self.node.append_child(child.node);
        self.state.update(&child.state);
    }

    fn prepend_child(&mut self, child: ParserState) {
        self.node.prepend_child(child.node);
        child.state.update(&self.state);
        self.state = child.state;
    }
}

pub struct Parser {
    root: ast::RootNode,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            root: ast::RootNode::new(),
        }
    }

    pub fn parse(&self, s: &str) -> Option<bool> {
        None
    }

    fn natural(&self, state: &State) -> Option<ParserState> {
        match seq!(ws!(), natural!()).matches(state) {
            None => None,
            Some(s) => Some(ParserState::new::<ast::Natural>(s)),
        }
    }

    fn number(&self, state: &State) -> Option<ParserState> {
        self.natural(&state)
    }

    fn value(&self, state: &State) -> Option<ParserState> {
        self.number(&state)
    }

    fn var(&self, state: &State) -> Option<ParserState> {
        match seq!(ws!(), opt!(chr!('$')), dotted_name!()).matches(&state) {
            None => None,
            Some(s) => Some(ParserState::new::<ast::VarRef>(s)),
        }
    }

    fn args(&self, state: &State) -> Option<ParserState> {
        let mut first_arg = self.expr(&state);
        if first_arg.is_none() {
            // void function
            return Some(ParserState {
                state: state.clone(),
                node: Box::new(ast::FnArgs { args: vec![] }),
            })
        }
        let mut args = vec![];
        let curr = first_arg.unwrap();
        args.push(curr.node);
        loop {
            let comma = seq!(ws!(), chr!(',')).matches(&curr.state);
            if comma.is_none() {
                return Some(ParserState {
                    state: curr.state,
                    node: Box::new(ast::FnArgs { args: args }),
                });
            }

            let arg = self.expr(&curr.state);
            if arg.is_none() { return None; }
        }
    }

    fn fcall(&self, state: &State) -> Option<ParserState> {
        let trimmed = ws!().matches(&state).unwrap();
        let dollar = opt!(chr!('$')).matches(&trimmed);
        let fn_name = dotted_name!().matches(&dollar.unwrap());
        if fn_name.is_none() { return None; }
        let lbrace = chr!('(').matches(&fn_name.as_ref().unwrap());
        if lbrace.is_none() { return None; }
        let args = self.args(&lbrace.unwrap());
        if args.is_none() { return None; }
        let rbrace = chr!(')').matches(&args.unwrap().state);
        if rbrace.is_none() { return None; }
        // Build the AST
        Some(ParserState{
            state: rbrace.unwrap(),
            node: Box::new(ast::FnCall{
                fn_name: fn_name.unwrap().matched().to_string(),
                args: args.unwrap().node,
            }),
        })
    }

    fn id(&self, state:&State) -> Option<ParserState> {
        let mut ret = self.fcall(&state);
        if ret.is_some() { return ret; }
        ret = self.value(&state);
        if ret.is_some() { return ret; }
        self.var(&state)
    }

    fn factor(&self, state: &State) -> Option<ParserState> {
        // Try matching an identifier
        let id = self.id(&state);
        if id.is_some() { return id; }
        // Match of identifier failed. Try match an expression between ()
        let lbrace = seq!(ws!(), chr!('(')).matches(&state);
        if lbrace.is_none() {return None;}
        let expr = self.expr(&lbrace.unwrap());
        if expr.is_none() { return None; }
        let rbrace = seq!(ws!(), chr!(')')).matches(&expr.as_ref().unwrap().state);
        if rbrace.is_none() { return None; }
        expr.unwrap().update(&rbrace.unwrap());
        expr
    }

    fn term_rhs(&self, state: &State) -> Option<ParserState> {
        let op = seq!(ws!(), alt!(chr!('*'), chr!('/'), chr!('%')))
            .matches(&state);
        if op.is_none() { return None; }
        let factor = self.factor(&op.as_ref().unwrap());
        if factor.is_none() { return None; }
        // Parsing done. Create ParserState
        let op = ParserState::new::<ast::BinaryOperator>(op.unwrap());
        op.append_child(factor.unwrap());
        Some(op)
    }

    fn term(&self, state: &State) -> Option<ParserState> {
        let mut ret = self.factor(&state);
        if ret.is_none() {
            return None;
        }
        loop {
            let curr = self.term_rhs(&ret.as_ref().unwrap().state);
            if curr.is_none() { return ret; }
            curr.unwrap().prepend_child(ret.unwrap());
            ret = curr;
        }
    }

    fn expr_rhs(&self, state: &State) -> Option<ParserState> {
        let op = seq!(ws!(), alt!(chr!('+'), chr!('-'))).matches(&state);
        if op.is_none() { return None; }
        let term = self.term(&op.as_ref().unwrap());
        if term.is_none() { return None; }
        // Parsing done. Create ParserState
        let op = ParserState::new::<ast::BinaryOperator>(op.unwrap());
        op.append_child(term.unwrap());
        Some(op)
    }

    fn expr(&self, state: &State) -> Option<ParserState> {
        let mut ret = self.term(&state);
        if ret.is_none() { return None; }
        loop {
            let curr = self.expr_rhs(&ret.as_ref().unwrap().state);
            if curr.is_none() { return ret; }
            curr.unwrap().prepend_child(ret.unwrap());
            ret = curr;
        }
    }

//    fn condition(&self, state: &State) -> Option<State> {
//        let mut trimmed = ws!().matches(&state).unwrap();
//        let not = chr!('!').matches(&trimmed);
//        if not.is_some() { return self.logic_factor(&not.unwrap()); }
//        let expr = self.expr(&trimmed);
//        if expr.is_none() { return None; }
//        trimmed = ws!().matches(&expr.unwrap()).unwrap();
//        let op = alt!(seq!(chr!('='), chr!('=')),
//                      seq!(chr!('!'), chr!('=')),
//                      seq!(chr!('<'), chr!('=')),
//                      seq!(chr!('>'), chr!('=')),
//                      chr!('<'),
//                      chr!('>')).matches(&trimmed);
//        if op.is_none() { return None; }
//        self.expr(&op.unwrap())
//    }
//
//    fn logic_factor(&self, state: &State) -> Option<State> {
//        let mut trimmed = ws!().matches(&state).unwrap();
//        let cond = self.condition(&trimmed);
//        if cond.is_some() {
//            return cond;
//        }
//        let lbrace = chr!('(').matches(&trimmed);
//        if lbrace.is_none() {return None;}
//        let expr = self.logic_expr(&lbrace.unwrap());
//        if expr.is_none() { return None; }
//        trimmed = ws!().matches(&expr.unwrap()).unwrap();
//        chr!(')').matches(&trimmed)
//    }
//
//    fn logic_term_rhs(&self, state: &State) -> Option<State> {
//        let trimmed = ws!().matches(&state).unwrap();
//        let op = seq!(chr!('&'), chr!('&')).matches(&trimmed);
//        if op.is_none() { return None; }
//        self.logic_factor(&op.unwrap())
//    }
//
//    fn logic_term(&self, state: &State) -> Option<State> {
//        let mut ret = self.logic_factor(&state);
//        if ret.is_none() { return None; }
//        loop {
//            let curr = self.logic_term_rhs(&ret.as_ref().unwrap());
//            if curr.is_none() { return ret; }
//            ret = curr;
//        }
//    }
//
//    fn logic_expr_rhs(&self, state: &State) -> Option<State> {
//        let trimmed = ws!().matches(&state).unwrap();
//        let op = seq!(chr!('|'), chr!('|')).matches(&trimmed);
//        if op.is_none() { return None; }
//        self.logic_term(&op.unwrap())
//    }
//
//    fn logic_expr(&self, state: &State) -> Option<State> {
//        let mut ret = self.logic_term(&state);
//        if ret.is_none() { return None; }
//        loop {
//            let curr = self.logic_expr_rhs(&ret.as_ref().unwrap());
//            if curr.is_none() { return ret; }
//            ret = curr;
//        }
//    }
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
    // Invalid number as 0 is followed by other digit. The parser gets the 0,
    // but does not parse the rest of the string
    let ret = p.number(&State::from_string("01234"));
    assert!(!ret.unwrap().complete());
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
fn parse_id() {
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

//#[test]
//fn parse_condition() {
//    let p = Parser::new();
//    assert_complete!(p.condition(&State::from_string("2 + 42 < 15 - 3")));
//    assert_complete!(p.condition(&State::from_string("2 + 42 <= 15 - 3")));
//    assert_complete!(p.condition(&State::from_string("2 + 42 > 15 - 3")));
//    assert_complete!(p.condition(&State::from_string("2 + 42 >= 15 - 3")));
//    assert_complete!(p.condition(&State::from_string("2 + 42 == 15 - 3")));
//    assert_complete!(p.condition(&State::from_string("2 + 42 != 15 - 3")));
//    assert_complete!(p.condition(&State::from_string("!(2 > 3)")));
//}
//
//#[test]
//fn parse_logic_term() {
//    let p = Parser::new();
//    assert_complete!(p.logic_term(&State::from_string("2 < 3 && 5 > 4")));
//}
//
//#[test]
//fn parse_logic_expr() {
//    let p = Parser::new();
//    assert_complete!(p.logic_expr(&State::from_string("2 < 3 || 5 > 4")));
//}
//
}
