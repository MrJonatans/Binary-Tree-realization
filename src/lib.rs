mod tree_tests;

use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Tree{
    root : Option<Box<Node>>,
} 

#[derive(Debug, PartialEq)]
struct Node{
    left : Option<Box<Node>>,
    right : Option<Box<Node>>,
    val : i64,
}

impl Node {
    fn new(val : i64) -> Self {
        Node{
            left : None,
            right : None,
            val,
        }
    }
}

impl From<Node> for Option<Box<Node>>{
    fn from(node : Node) -> Self {
        Some(Box::new(node))
    }
}

impl fmt::Display for Tree{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.root{
            None =>{
                panic!("Attemp to display an empty tree!")
            },
            Some(_node)=>{
                Tree::print_tree(&self.root, 0, f)
            }
        }
    }
}


impl Tree{
    pub fn new() -> Self {
        Tree{root: None}
    }

    pub fn insert(&mut self, val: i64){
        match &mut self.root{
            None => {
                self.root = Node::new(val).into();
            },
            Some(node) =>{
                 Tree::rec_insert(node, val);
            }
         }
    }

    pub fn del(&mut self, val : i64){
        match self.root.take() {
            None => {
                panic!("Attemp to delete from emty tree!")
            },
            Some(_node) => {
                self.root = Tree::delete_el(_node, val)
            }
        }
    }

    pub fn max(&mut self)->i64{
        match &mut self.root {
            None => {
                panic!("Attemp find max from emty tree!")
            },
            Some(_node) => {
                Tree::get_max(&_node)
            }
        }
    }

    fn rec_insert(node : &mut Box<Node>, val: i64) {
        if val < node.val {
            match &mut node.left {
                None => {
                    node.left = Node::new(val).into();
                },
                Some(_node) => {
                    Tree::rec_insert(_node, val);
                }
            }
        } else if val > node.val {
            match &mut node.right {
                None => {
                    node.right = Node::new(val).into();
                },
                Some(_node) => {
                    Tree::rec_insert(_node, val);
                }
            }
        }
    }

    #[warn(unused_must_use)]
    fn print_tree(node :& Option<Box<Node>>, tab_val : i64,f :&mut fmt::Formatter<'_>) -> fmt::Result{
        match &node{
            None => {
                write!(f,"")
            },
            Some(_node) => {
                let _ = Tree::print_tree(&_node.left, tab_val+1, f);
                for _n in 1..=tab_val {print!(" ")};
                let hehe = writeln!(f,"{} ",_node.val); //второй возможно_костыль
                let _ = Tree::print_tree(&_node.right,tab_val+1, f);
                hehe
            }
        }
    }

    fn delete_el(mut node :Box<Node>, val : i64) -> Option<Box<Node>>{
        if node.val > val {
            match node.left{
                None => {
                    panic!("Attemp to delete non-existent element!");
                },
                Some(_node) =>{
                    node.left = Tree::delete_el(_node, val);
                    Some(node)
                }
            }
        }
        else if node.val < val {
            match node.right{
                None => {
                    panic!("Attemp to delete non-existent element!");
                },
                Some(_node) =>{
                    node.right = Tree::delete_el(_node, val);
                    Some(node)
                }
            }
        }
        else {
            match (node.left, node.right){
                (None,None) => {
                    None
                },
                (None,Some(node_right)) => {
                    Some(node_right)
                },
                (Some(node_left), None)=>{
                    Some(node_left)
                },
                (Some(mut _node_left),Some(_node_right)) =>{
                    let left_max = Tree::get_max(&_node_left);
                    node.val = left_max;
                    node.left = Tree::delete_el(_node_left, left_max);
                    node.right = _node_right.into(); //передаю владение переменной обратно, ах"!@ ход
                    Some(node)
                }
            }
        } 
    }

    fn get_max(node :& Box<Node>) -> i64{
        let mut result = node;
        while result.right!=None {
            if let Some(tmp) = result.right.as_ref(){
                result = tmp;
            }
        }
        result.val
    }
}


