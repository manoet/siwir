// Copyright (c) 2020 Marco Giglio
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

pub trait AstNode {

}

pub trait StringInitialized {
    fn from_str(v: &str) -> Box<dyn AstNode>;
}

pub struct RootNode {
    root: Option<Box<dyn AstNode>>,
}

impl RootNode {
    pub fn new() -> RootNode {
        RootNode {
            root: None,
        }
    }
}

struct BinaryOperator {
    op: String,
    left: Option<Box<dyn AstNode>>,
    right: Option<Box<dyn AstNode>>,
}

pub struct UnaryOperator {
    arg: Option<Box<dyn AstNode>>,
}

pub struct FnCall {
    pub fn_name: String,
    //args: FnArgs,
}

impl AstNode for FnCall {}

pub struct FnArgs {
    args: Vec<Box<dyn AstNode>>,
}

pub struct VarRef {
    var_name: String,
}

impl AstNode for VarRef {}

impl StringInitialized for VarRef {
    fn from_str(v: &str) -> Box<dyn AstNode> {
        Box::new(VarRef {var_name: v.to_string()})
    }
}

pub struct Natural {
    pub value: u32,
}

impl AstNode for Natural {}

impl StringInitialized for Natural {
    fn from_str(v: &str) -> Box<dyn AstNode> {
        Box::new(Natural {value: v.parse::<u32>().unwrap()})
    }
}
