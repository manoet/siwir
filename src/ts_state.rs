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

#[test]
fn peek_one() {
    let s = State::from_string("Hello, World!");
    assert_eq!(s.peek(), 'H');
}

#[test]
fn peek_one_exceed() {
    let s = State::from_string("");
    assert_eq!(s.peek(), '\0');
}

#[test]
fn peek_many() {
    let s = State::from_string("Hello, World!");
    assert_eq!(s.peek_many(7), "Hello, ");
}

#[test]
fn peek_many_exceed() {
    let s = State::from_string("Hi");
    assert_eq!(s.peek_many(3), "");
}

#[test]
fn complete_true() {
    let s = State::from_string("");
    assert!(s.complete());
}

#[test]
fn complete_false() {
    let s = State::from_string("A");
    assert_eq!(s.complete(), false);
}

#[test]
fn read() {
    let s1 = State::from_string("Hello, World!");
    let s2 = s1.read(7);
    assert_eq!(s2.peek_many(6), "World!");
}

