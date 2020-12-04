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

#[test]
fn chr_match() {
    let matcher = chr!('a');
    let result = matcher.matches(&State::from_string("abcd"));
    assert!(result.is_some());
    let result = result.unwrap();
    assert_eq!(result.peek_many(3), "bcd");
    assert_eq!(result.matched(), "a");
}

#[test]
fn chr_no_match() {
    let matcher = chr!('a');
    let result = matcher.matches(&State::from_string("ABCDabcd"));
    assert!(result.is_none());
}

#[test]
fn range_chr_match() {
    let matcher = chr!('0', '9');
    let result = matcher.matches(&State::from_string("0"));
    assert!(result.is_some());
    assert_eq!(result.unwrap().matched(), "0");
    let result = matcher.matches(&State::from_string("5"));
    assert!(result.is_some());
    assert_eq!(result.unwrap().matched(), "5");
    let result = matcher.matches(&State::from_string("9"));
    assert!(result.is_some());
    assert_eq!(result.unwrap().matched(), "9");
}

#[test]
fn range_chr_no_match() {
    let matcher = chr!('0', '9');
    let result = matcher.matches(&State::from_string("ABCD1234"));
    assert!(result.is_none());
}

#[test]
fn seq() {
    let matcher = seq!(chr!('a'), chr!('b'), chr!('c'));
    let result = matcher.matches(&State::from_string("abcd"));
    assert!(result.is_some());
    let result = result.unwrap();
    assert_eq!(result.peek(), 'd');
    assert_eq!(result.matched(), "abc");
}

#[test]
fn alt_match() {
    let state = State::from_string("abc");
    let matcher1 = alt!(chr!('a'), chr!('b'));
    let matcher2 = alt!(chr!('b'), chr!('a'));
    let result1 = matcher1.matches(&state);
    assert!(result1.is_some());
    assert_eq!(result1.as_ref().unwrap().peek(), 'b');
    assert_eq!(result1.unwrap().matched(), "a");
    let result2 = matcher2.matches(&state);
    assert!(result2.is_some());
    assert_eq!(result2.as_ref().unwrap().peek(), 'b');
    assert_eq!(result2.unwrap().matched(), "a");
}

#[test]
fn alt_no_match() {
    let state = State::from_string("cba");
    let matcher = alt!(chr!('a'), chr!('b'));
    let result = matcher.matches(&state);
    assert!(result.is_none());
}

#[test]
fn opt() {
    let matcher = opt!(chr!('b'));
    // For opt being false
    let result= matcher.matches(&State::from_string("abc"));
    assert!(result.is_some());
    assert_eq!(result.as_ref().unwrap().peek(), 'a');
    assert_eq!(result.unwrap().matched(), "");
    // for opt being true
    let result = matcher.matches(&State::from_string("bcd"));
    assert!(result.is_some());
    assert_eq!(result.as_ref().unwrap().peek(), 'c');
    assert_eq!(result.unwrap().matched(), "b");
}

#[test]
fn rep() {
    // Testing ?
    let matcher = rep!(chr!('a'), '?');
    let mut result = matcher.matches(&State::from_string("abcd"));
    assert!(result.is_some());
    result = matcher.matches(&State::from_string("bcda"));
    assert!(result.is_some());
    assert_eq!(result.unwrap().matched(), "");
    // Testing +
    let matcher = rep!(chr!('a'), '+');
    result = matcher.matches(&State::from_string("aabcd"));
    assert!(result.is_some());
    assert_eq!(result.as_ref().unwrap().peek(), 'b');
    assert_eq!(result.unwrap().matched(), "aa");
    result = matcher.matches(&State::from_string("bcda"));
    assert!(result.is_none());
    // Testing *
    let matcher = rep!(chr!('a'), '*');
    result = matcher.matches(&State::from_string("aaaaaaabcd"));
    assert!(result.is_some());
    assert_eq!(result.as_ref().unwrap().peek(), 'b');
    assert_eq!(result.unwrap().matched(), "aaaaaaa");
    result = matcher.matches(&State::from_string("bcda"));
    assert!(result.is_some());
    assert_eq!(result.as_ref().unwrap().matched(), "");
    assert_eq!(result.unwrap().peek(), 'b');
}

#[test]
fn lower_letter() {
    let matcher = lower_letter!();
    let mut result = matcher.matches(&State::from_string("a"));
    assert!(result.is_some());
    result = matcher.matches(&State::from_string("z"));
    assert!(result.is_some());
    result = matcher.matches(&State::from_string("A"));
    assert!(result.is_none());
}

#[test]
fn upper_letter() {
    let matcher = upper_letter!();
    let mut result = matcher.matches(&State::from_string("A"));
    assert!(result.is_some());
    result = matcher.matches(&State::from_string("Z"));
    assert!(result.is_some());
    result = matcher.matches(&State::from_string("a"));
    assert!(result.is_none());
}

#[test]
fn letter() {
    let matcher = letter!();
    let mut result = matcher.matches(&State::from_string("A"));
    assert!(result.is_some());
    result = matcher.matches(&State::from_string("Z"));
    assert!(result.is_some());
    result = matcher.matches(&State::from_string("a"));
    assert!(result.is_some());
    result = matcher.matches(&State::from_string("z"));
    assert!(result.is_some());
    result = matcher.matches(&State::from_string("_"));
    assert!(result.is_none());
    result = matcher.matches(&State::from_string("0"));
    assert!(result.is_none());
}

#[test]
fn alpha() {
    let matcher = alpha!();
    let mut result = matcher.matches(&State::from_string("A"));
    assert!(result.is_some());
    result = matcher.matches(&State::from_string("z"));
    assert!(result.is_some());
    result = matcher.matches(&State::from_string("_"));
    assert!(result.is_some());
    result = matcher.matches(&State::from_string("0"));
    assert!(result.is_none());
}

#[test]
fn digit() {
    let matcher = digit!();
    let mut result = matcher.matches(&State::from_string("0"));
    assert!(result.is_some());
    result = matcher.matches(&State::from_string("9"));
    assert!(result.is_some());
    result = matcher.matches(&State::from_string("a"));
    assert!(result.is_none());
}

#[test]
fn letter_str() {
    let matcher = letter_str!();
    let mut result = matcher.matches(&State::from_string("Hello"));
    assert!(result.is_some());
    assert!(result.unwrap().complete());
    let mut result = matcher.matches(&State::from_string("Hello_"));
    assert!(result.is_some());
    assert_eq!(result.as_ref().unwrap().matched(), "Hello");
    assert_eq!(result.unwrap().peek(), '_');
    result = matcher.matches(&State::from_string("Alpha42"));
    assert!(result.is_some());
    assert_eq!(result.as_ref().unwrap().matched(), "Alpha");
    assert!(result.unwrap().peek() == '4');
}

#[test]
fn alpha_str() {
    let matcher = alpha_str!();
    let mut result = matcher.matches(&State::from_string("Hello"));
    assert!(result.is_some());
    assert_eq!(result.as_ref().unwrap().matched(), "Hello");
    assert!(result.unwrap().complete());
    let mut result = matcher.matches(&State::from_string("Hello_"));
    assert!(result.is_some());
    assert_eq!(result.as_ref().unwrap().matched(), "Hello_");
    assert!(result.unwrap().complete());
    result = matcher.matches(&State::from_string("Alpha42"));
    assert!(result.is_some());
    assert_eq!(result.as_ref().unwrap().matched(), "Alpha");
    assert!(result.unwrap().peek() == '4');
}

#[test]
fn alphanum_str() {
    let matcher = alphanum_str!();
    let mut result = matcher.matches(&State::from_string("Hello"));
    assert!(result.is_some());
    assert_eq!(result.as_ref().unwrap().matched(), "Hello");
    assert!(result.unwrap().complete());
    let mut result = matcher.matches(&State::from_string("Hello_"));
    assert!(result.is_some());
    assert_eq!(result.as_ref().unwrap().matched(), "Hello_");
    assert!(result.unwrap().complete());
    result = matcher.matches(&State::from_string("Alpha42"));
    assert!(result.is_some());
    assert_eq!(result.as_ref().unwrap().matched(), "Alpha42");
    assert!(result.unwrap().complete());
}
