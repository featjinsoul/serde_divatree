use slab_tree::*;

use std::ops::Deref;

pub mod serde;

use crate::error::ParseError;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Node<'a> {
    Key(&'a str),
    Value(&'a str),
}

struct A3daTree<'a> {
    tree: Tree<Node<'a>>,
}

impl<'a> A3daTree<'a> {
    fn new(input: &'a str) -> Result<A3daTree, ParseError> {
        let lines = input.lines();
        let mut tree = TreeBuilder::new().with_root(Node::Key("a3da_root")).build();

        for (line_num, line) in lines.enumerate() {
            let mut splits = line.split('=');
            let lhs = splits
                .next()
                .ok_or_else(|| ParseError { line, line_num })?
                .trim();
            let sections = lhs.split('.');
            let rhs = splits
                .next()
                .ok_or_else(|| ParseError { line, line_num })?
                .trim();

            //Guarranteed not to panic, since root is always in tree
            let mut id = tree.root_id().unwrap();

            for sec in sections {
                let node = tree.get(id).unwrap();
                match node.children().find(|x| x.data().deref() == sec) {
                    Some(n) => {
                        id = n.node_id();
                        continue;
                    }
                    None => {
                        let mut node = tree.get_mut(id).unwrap();
                        let new = node.append(Node::Key(sec)).node_id();
                        id = new;
                    }
                }
            }
            tree.get_mut(id).unwrap().append(Node::Value(rhs));
        }
        Ok(Self { tree })
    }
}

impl<'a> std::ops::Deref for Node<'a> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Key(v) => v,
            Self::Value(v) => v,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "camera_root.0.interest.trans.x.key.0.data=(0,-0.469822)
camera_root.0.interest.trans.x.key.0.type=1
camera_root.0.interest.trans.x.key.1.data=(738,-0.522281,3.31402e-006)
";

    #[test]
    fn new() {
        let tree = A3daTree::new(INPUT).unwrap();
        let mut str = String::new();
        tree.tree.write_formatted(&mut str).unwrap();
        println!("{}", str);
        println!("----------- WRITE -------------");
        panic!();
    }
}
