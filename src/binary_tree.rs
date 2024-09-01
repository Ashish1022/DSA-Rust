// binary tree module

use std::{collections::VecDeque, vec};

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

    fn level_order_traversal(&self) -> Vec<i32> {
        if self.root.is_none(){
            return Vec::new();
        }

        let mut results: Vec<i32> = Vec::new();
        let mut q: VecDeque<&Box<Node>> = VecDeque::new();
        let root = self.root.as_ref().unwrap();
        results.push(root.value);
        q.push_back(root);


        let mut height = 0;
        while !q.is_empty(){
            for i in 0..q.len(){
                if let Some(node) = q.pop_front(){
                    if let Some(ref left) = node.left_node{
                        results.push(left.value);
                        q.push_back(left);
                    }
                    if let Some(ref right) = node.right_node{
                        results.push(right.value);
                        q.push_back(right);
                    }
                }
            }

            height += 1;
        }

        println!("Height: {height}");

        results
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

    #[test]
    fn works_build_level_traversal() {
        let mut tree = Tree::new();
        tree.insert(8);
        tree.insert(10);
        tree.insert(3);
        tree.insert(12);
        tree.insert(6);
        tree.insert(1);
        tree.insert(4);


        assert_eq!(tree.level_order_traversal(), vec![8, 3, 10, 1, 6, 12, 4]);
    }
}