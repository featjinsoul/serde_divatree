use slab_tree::*;

use std::ops::Deref;

pub mod serde;
pub use self::serde::from_str;

use crate::error::ParseError;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Node<'a> {
    Key(&'a str),
    Value(&'a str),
}

struct A3daTree<'a> {
    tree: Tree<Node<'a>>,
    curr: NodeId,
}

impl<'a> A3daTree<'a> {
    fn new(input: &'a str) -> Result<A3daTree, ParseError> {
        let lines = input.lines();
        let mut tree = TreeBuilder::new().with_root(Node::Key("a3da_root")).build();

        for (line_num, line) in lines.enumerate() {
            if line.is_empty() || line.starts_with('$') {
                continue;
            }
            let mut splits = line.split('=');
            let err = || ParseError {
                line_num,
                line: line.to_string(),
            };
            let lhs = splits.next().ok_or_else(err)?.trim();
            let sections = lhs.split('.');
            let rhs = splits.next().ok_or_else(err)?.trim();

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
        let curr = tree
            .root()
            .unwrap()
            .first_child()
            .unwrap_or(tree.root().unwrap())
            .node_id();
        Ok(Self { tree, curr })
    }
    fn print(&self) {
        let mut str = String::new();
        self.tree.write_formatted(&mut str).unwrap();
        println!("{}", str);
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

    pub(crate) const INPUT: &'static str = "camera_root.0.interest.trans.x.key.0.data=(0,-0.469822)
camera_root.0.interest.trans.x.key.0.type=1
camera_root.0.interest.trans.x.key.1.data=(738,-0.522281,3.31402e-006)
camera_root.0.interest.trans.x.key.1.type=2
";

    #[test]
    fn new() {
        let tree = A3daTree::new(INPUT).unwrap();
        tree.print();
        println!("----------- WRITE -------------");
        panic!();
    }
}
