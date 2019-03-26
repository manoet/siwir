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

