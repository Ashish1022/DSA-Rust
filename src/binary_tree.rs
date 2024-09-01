// binary tree module

use std::vec;

#[derive(Debug)]
struct Tree {
    root: Option<Box<Node>>,
}

#[derive(Debug)]
struct Node {
    value: i32,
    left_node: Option<Box<Node>>,
    right_node: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32) -> Self{
        Node{
            value,
            left_node: None,
            right_node: None,
        }
    }
}

impl From<Node> for Option<Box<Node>>{
    fn from(node: Node) -> Self {
        Some(Box::new(node))
    }
}

impl Tree{
    fn new() -> Self{
        Tree {
            root: None,
        }
    }

    fn insert(&mut self, value: i32) {
        match &mut self.root {
            None => {
                self.root = Node::new(value).into();
            },
            Some(node) => {
                Tree::insert_recursive(node, value);
            }
        }
    }

    fn insert_iterative(&mut self, value: i32) {
        if self.root.is_none(){
            self.root = Node::new(value).into();
            return;
        }

        let mut q: Vec<&mut Box<Node>> = Vec::new();
        let root = self.root.as_mut().unwrap();
        q.push(root);
        
        while let Some(node) = q.pop(){
            if value > node.value{
                match &mut node.right_node {
                    None => {
                        node.right_node = Node::new(value).into();
                    },
                    Some(n) => {

                    }
                }
            }else if value > node.value{
                match &mut node.left_node {
                    None => {
                        node.left_node = Node::new(value).into();
                    },
                    Some(n) => {
                        
                    }
                }
            }
        }
    }

    fn insert_recursive(node: &mut Box<Node>, value: i32) {
        if value > node.value{
            match &mut node.right_node{
                None => {
                    node.right_node = Node::new(value).into();
                },
                Some(n) => {
                    Tree::insert_recursive(n, value);
                }
            }
        } else if value < node.value {
            match &mut node.left_node {
                None => {
                    node.left_node = Node::new(value).into();
                },
                Some(n) => {
                    Tree::insert_recursive(n, value);
                }
            }
        }
    }
}

#[cfg(test)]

mod tests {
    use super::*;
    #[test]
    fn works_build_tree(){
        let mut tree = Tree::new();
        tree.insert(8);
        tree.insert(10);
        tree.insert(3);
        tree.insert(12);
        tree.insert(6);
        tree.insert(1);
        tree.insert(4);

        assert_eq!(tree.root.is_some(), true);
        println!("{:?}", tree);
    }
}