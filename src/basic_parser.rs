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

#[derive(Clone)]
pub struct State {
    s: String,
}

impl State {
    pub fn from_string(s: &str) -> State {
        State {
            s: s.to_string(),
        }
    }

    pub fn display(&self) -> &str {
        &self.s[..]
    }

    pub fn peek(&self) -> char {
        match self.s.chars().next() {
            Some(c) => c,
            None => '\0',
        }
    }

    pub fn peek_many(&self, n: usize) -> &str {
        if n > self.s.len() {
            return "";
        }
        &self.s[..n]
    }

    pub fn read(&self, n: usize) -> State {
        State {
            s: self.s[n..].to_string(),
        }
    }

    pub fn complete(&self) -> bool {
        self.s.len() == 0
    }
}


pub struct Transition<T> where T: Fn(&State) -> Option<State>, {
    func: T,
}

impl<T> Transition<T> where T: Fn(&State) -> Option<State>, {
    pub fn new(func: T) -> Transition<T> {
        Transition {
            func,
        }
    }

    pub fn matches(&self, state: &State) -> Option<State> {
        (self.func)(state)
    }
}

macro_rules! chr {
    ($c:expr) => {
        chr!($c, $c)
    };
    ($c1:expr, $c2:expr) => {
        {
            let trans = Transition::new(move |state| {
                let curr = state.peek();
                if curr >= $c1 && curr <= $c2 {
                    return Some(state.read(1));
                }
                None
            });
            trans
        }
    };
}

macro_rules! seq {
    ($m1:expr, $($m2:expr),*) => {
        {
            let trans = Transition::new(move |state| {
                let ret = $m1.matches(state);
                if ret.is_none() {
                    return None;
                }
                $(
                    // Now match the other matchers
                    let state = ret.unwrap();
                    let ret = $m2.matches(&state);
                    if ret.is_none() {
                        return None;
                    }
                )*
                ret
            });
            trans
        }
    };
}

macro_rules! opt {
    ($m:expr) => {
        {
            let trans = Transition::new(move |state| {
                let ret = $m.matches(state);
                match ret {
                    Some(_) => ret,
                    None => Some(state.clone()),
                }
            });
            trans
        }
    };
}

macro_rules! any {
    ($m:expr) => {
        {
            let trans = Transition::new(move |state| {
                let mut prev = state.clone();
                loop {
                    let ret = $m.matches(&prev);
                    if ret.is_some() {
                        prev = ret.unwrap();
                    } else {
                        return Some(prev);
                    }
                }
            });
            trans
        }
    };
}

macro_rules! rep {
    ($matcher:expr, $times:expr) => {
        {
            let trans = Transition::new(move |state| {
                match $times {
                    '?' => opt!($matcher).matches(state),
                    '+' => {
                        let ret = $matcher.matches(state);
                        match ret {
                            Some(_) => any!($matcher).matches(&ret.unwrap()),
                            None => None,
                        }
                    },
                    '*' => any!($matcher).matches(state),
                    _ => panic!("Unknown quantifier {}", $times),
                }
            });
            trans
        }
    };
}

macro_rules! alt {
    ($($m:expr),+) => {
        {
            let trans = Transition::new(move |state| {
                $(
                    let ret = $m.matches(state);
                    if ret.is_some() {
                        return ret;
                    }
                )*
                None
            });
            trans
        }
    }
}

///////////////////////////////////////////////////////////////////////
// Complex macro
///////////////////////////////////////////////////////////////////////

macro_rules! lower_letter {
    () => {
        chr!('a', 'z')
    }
}

macro_rules! upper_letter {
    () => {
        chr!('A', 'Z')
    }
}

macro_rules! letter {
    () => {
        alt!(lower_letter!(), upper_letter!())
    }
}

macro_rules! alpha {
    () => {
        alt!(letter!(), chr!('_'))
    }
}

macro_rules! digit {
    () => {
        chr!('0', '9')
    }
}

macro_rules! letter_str {
    () => {
        rep!(letter!(), '+')
    }
}

macro_rules! alpha_str {
    () => {
        rep!(alpha!(), '+')
    }
}

macro_rules! alphanum_str {
    () => {
        rep!(alt!(alpha!(), digit!()), '+')
    }
}

