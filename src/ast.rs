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
    fn append_child(&mut self, node: Box<dyn AstNode>);
    fn prepend_child(&mut self, node: Box<dyn AstNode>);
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

impl AstNode for RootNode {
    fn append_child(&mut self, node: Box<dyn AstNode>) {
        if self.root.is_some() {
            panic!("Root node already set.");
        }
        self.root = Some(node);
    }

    fn prepend_child(&mut self, _node: Box<dyn AstNode>) {
        panic!("Prepend not implemented for root node.");
    }
}

pub struct BinaryOperator {
    op: String,
    left: Option<Box<dyn AstNode>>,
    right: Option<Box<dyn AstNode>>,
}

impl AstNode for BinaryOperator {
    fn append_child(&mut self, node: Box<dyn AstNode>) {
        if self.right.is_some() {
            panic!("Right node is already set.");
        }
        self.right = Some(node);
    }

    fn prepend_child(&mut self, node: Box<dyn AstNode>) {
        if self.left.is_some() {
            panic!("Left node is already set.");
        }
        self.left = Some(node);
    }
}

impl StringInitialized for BinaryOperator {
    fn from_str(v: &str) -> Box<dyn AstNode> {
        Box::new(BinaryOperator {
            op: v.to_string(),
            left: None,
            right: None
        })
    }
}

pub struct UnaryOperator {
    arg: Option<Box<dyn AstNode>>,
}

pub struct FnCall {
    pub fn_name: String,
    pub args: Box<dyn AstNode>,
}

impl AstNode for FnCall {
    fn append_child(&mut self, node: Box<dyn AstNode>) {
        panic!("Append not implemented in FnCall node.");
    }

    fn prepend_child(&mut self, node: Box<dyn AstNode>) {
        panic!("Prepend not implemented in FnCall node.");
    }
}

pub struct FnArgs {
    args: Vec<Box<dyn AstNode>>,
}

impl AstNode for FnArgs {
    fn append_child(&mut self, node: Box<dyn AstNode>) {
        self.args.push(node);
    }

    fn prepend_child(&mut self, node: Box<dyn AstNode>) {
        panic!("Prepend not implemented in FnArgs node.");
    }
}

pub struct VarRef {
    var_name: String,
}

impl AstNode for VarRef {
    fn append_child(&mut self, node: Box<dyn AstNode>) {
        panic!("Append not implemented in VarRef node.");
    }
    fn prepend_child(&mut self, node: Box<dyn AstNode>) {
        panic!("Prepend not implemented in VarRef node.");
    }
}

impl StringInitialized for VarRef {
    fn from_str(v: &str) -> Box<dyn AstNode> {
        Box::new(VarRef {var_name: v.to_string()})
    }
}

pub struct Natural {
    pub value: u32,
}

impl AstNode for Natural {
    fn append_child(&mut self, node: Box<dyn AstNode>) {
        panic!("Append not implemented in Natural node.");
    }
    fn prepend_child(&mut self, node: Box<dyn AstNode>) {
        panic!("Prepend not implemented in Natural node.");
    }
}

impl StringInitialized for Natural {
    fn from_str(v: &str) -> Box<dyn AstNode> {
        Box::new(Natural {value: v.parse::<u32>().unwrap()})
    }
}
