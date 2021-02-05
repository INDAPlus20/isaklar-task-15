use core::panic;
use std::usize;

fn main() {

}
#[derive(PartialEq, Debug)]
enum Color {
    Black,
    Red
}
struct RedBlackTree {
    nodes: Vec<Node>
}

impl RedBlackTree {
    pub fn get_parent(&self, node: usize) -> Option<usize> {
        return self.nodes[node].parent;
    }

    pub fn get_grand_parent(&self, node: usize) -> Option<usize> {
        if let Some(parent) = self.nodes[node].parent{
            return self.nodes[parent].parent ;
        } else {
            return None;
        }
        
    }
    
    pub fn get_sibling(&self, node: usize) -> Option<usize> {
        if let Some(parent) = self.nodes[node].parent{ 
            
            if let Some(left) = self.nodes[parent].left {
                if left != node {
                    return Some(left);
                }
            } 
            if let Some(right) = self.nodes[parent].right {
                if right != node {
                    return Some(right);
                }
            }
            return None;
        } else {
            return None
        }
    }

    pub fn get_uncle(&self, node: usize) -> Option<usize> {
        if let Some(parent) = self.get_parent(node) {
            return self.get_sibling(parent);
        } else {
            return None;
        }
        
    }

    pub fn rotate_left(&mut self, node: usize) -> Result<usize, &'static str> {
        // check if possible to rotate left
        if let Some(nnew) = self.nodes[node].right {
            let p = self.get_parent(node);

            // update pointers
            self.nodes[node].right = self.nodes[nnew].left;
            self.nodes[nnew].left = Some(node);
            self.nodes[node].parent = Some(nnew);

            if let Some(right) = self.nodes[node].right {
                self.nodes[right].parent = Some(node)
            }

            // if the node had a parent
            if let Some(parent) = p {
                if let Some(left) = self.nodes[parent].left {
                    if left == node {
                        self.nodes[parent].left = Some(nnew);
                    } 
                } 

                if let Some(right) = self.nodes[parent].right {
                    if right == node {
                        self.nodes[parent].right = Some(nnew);
                    }
                }
                
            }
            // update new node parent
            self.nodes[nnew].parent = p;
            return Ok(nnew);   
        } else {
            return Err ("No right node");
        }
    }
    pub fn rotate_right(&mut self, node: usize) -> Result<usize, &'static str> {
        // check if possible to rotate left
        if let Some(nnew) = self.nodes[node].left {
            let p = self.get_parent(node);

            // update pointers
            self.nodes[node].left = self.nodes[nnew].right;
            self.nodes[nnew].right = Some(node);
            self.nodes[node].parent = Some(nnew);

            if let Some(left) = self.nodes[node].left {
                self.nodes[left].parent = Some(node)
            }

            // if the node had a parent
            if let Some(parent) = p {
                if let Some(right) = self.nodes[parent].right {
                    if right == node {
                        self.nodes[parent].right = Some(nnew);
                    } 
                } 

                if let Some(left) = self.nodes[parent].left {
                    if left == node {
                        self.nodes[parent].left = Some(nnew);
                    }
                }
                
            }
            // update new node parent
            self.nodes[nnew].parent = p;
            return Ok(nnew);   
        } else {
            return Err ("No right node");
        }
    }

    pub fn insert(&mut self, node: Node, root: Option<usize> ) -> usize {
        // push the node onto the list and get its index
        self.nodes.push(node);
        let node = self.nodes.len()-1;
        // insert into tree
        self.insert_recursive(root, node);

        // repair tree
        self.repair_tree(node);
        // find new root
        let mut root = node;
        while let Some(parent) = self.get_parent(root) {
            root = parent;
        }
        return root;
    }

    pub fn insert_recursive(&mut self, root: Option<usize>, node: usize) {
        // Descend the tree until a leaf is found
        if let Some(root) = root{
            if self.nodes[node].key < self.nodes[root].key {
                if let Some(left) = self.nodes[root].left {
                    self.insert_recursive(Some(left), node);
                    return;
                } else{
                    self.nodes[root].left = Some(node);
                } 

            } else {
                if let Some(right) = self.nodes[root].right {
                    self.insert_recursive(Some(right), node);
                    return;
                } else {
                    self.nodes[root].right = Some(node);
                }
            }
        }

        // leaf was found, update pointers
        self.nodes[node].parent = root;
        self.nodes[node].left = None;
        self.nodes[node].right = None;
        self.nodes[node].color = Color::Red;
    }

    pub fn repair_tree(&mut self, node: usize) {
        if let Some(parent) = self.get_parent(node){
            //println!("parent is: {:?}", parent);
            if self.nodes[parent].color == Color::Black {
                // Do nothing, tree is valid

            } else if self.get_uncle(node) != None && self.nodes[self.get_uncle(node).unwrap()].color == Color::Red{
                // Uncle and parent are red, repaint all the way up until tree is valid
                self.nodes[parent].color = Color::Black;
                let uncle = self.get_uncle(node).unwrap();
                self.nodes[uncle].color = Color::Black;
                let grand_parent = self.get_parent(parent).unwrap();
                self.nodes[grand_parent].color = Color::Red;
                self.repair_tree(grand_parent); 

            } else {
                // Parent is red but uncle is black
                let mut grand_parent = self.get_parent(parent).unwrap();
                let mut node = node;
                let p_right = self.nodes[parent].right;
                let p_left = self.nodes[parent].left;
                let g_right = self.nodes[grand_parent].right;
                let g_left = self.nodes[grand_parent].left;
                if p_right != None && g_left != None && node == p_right.unwrap() && parent == g_left.unwrap() {
                    self.rotate_left(parent);
                    node = self.nodes[node].left.unwrap();
                    
                } else if p_left != None && g_right != None && node == p_left.unwrap() && parent == g_right.unwrap(){
                    self.rotate_right(parent);
                    node = self.nodes[node].right.unwrap();
                }

                // part 2
                let parent = self.get_parent(node).unwrap();
                grand_parent = self.get_parent(parent).unwrap();
                let p_left = self.nodes[parent].left;
                if p_left != None && node == p_left.unwrap(){
                    self.rotate_right(grand_parent);
                } else {
                    self.rotate_left(grand_parent);
                }
                self.nodes[parent].color = Color::Black;
                self.nodes[grand_parent].color = Color::Red;
            }
        } else {
            // case 1
            self.nodes[node].color = Color::Black;
        }
    }

    pub fn remove(&mut self, node: usize){
        let mut children = 0;
        if self.nodes[node].left != None {
            children += 1;
        }
        if self.nodes[node].right != None {
            children +=1;
        }
        match children {
            0 => {
                
            },
            1 => {},
            2 => {},
            _ => panic!("wtf")
        }
    }

    fn no_child_remove(&mut self, node: usize) {
        if let Some(parent) = self.get_parent(node) {
            if let Some(left) = self.nodes[parent].left {
                if left == node {
                    self.nodes[parent].left = None;
                }
            } else {
                self.nodes[parent].right = None;
            }
        }
        
        self.nodes[node].parent = None;

        // heres when i would delete the node but im using the order in the nodes Vec as pointers
        // so deleting an element would mess up the whole system
        // don't judge me
    }

}

struct Node {
    parent:  Option<usize>,
    left: Option<usize>,
    right: Option<usize>,
    color: Color,
    key: i32
}

impl Node {
    pub fn new(key: i32) -> Node {
        Node {
            parent: None,
            left: None,
            right: None,
            color: Color::Red,
            key: key
        }
    }
}

#[cfg(test)]

mod test{
    use super::*;
    #[test]
    fn init_insert() {
        let mut tree = RedBlackTree {
            nodes: Vec::new()
        };
        tree.insert(Node::new(10), None);
        println!("{:?}", tree.nodes[0].key);

    }

    #[test]
    fn multiple_sorted_inserts() {
        let mut tree = RedBlackTree{
            nodes: Vec::new()
        };
        let mut root = tree.insert(Node::new(10), None);
        println!("root: {:?}, color: {:?}", root, tree.nodes[root].color);
        root = tree.insert(Node::new(12), Some(root));
        println!("root: {:?}, color: {:?}", root, tree.nodes[root].color);
        root = tree.insert(Node::new(13), Some(root));
        println!("root: {:?}, color: {:?}", root, tree.nodes[root].color);
        root = tree.insert(Node::new(14), Some(root));
        println!("root: {:?}, color: {:?}", root, tree.nodes[root].color);
        
    }
    #[test]
    fn same_key_inserts() {
        let mut tree = RedBlackTree{
            nodes: Vec::new()
        };
        let mut root = tree.insert(Node::new(13), None);
        println!("root: {:?}, color: {:?}", root, tree.nodes[root].color);
        root = tree.insert(Node::new(13), Some(root));
        println!("root: {:?}, color: {:?}", root, tree.nodes[root].color);
        root = tree.insert(Node::new(13), Some(root));
        println!("root: {:?}, color: {:?}", root, tree.nodes[root].color);
        root = tree.insert(Node::new(13), Some(root));
        println!("root: {:?}, color: {:?}", root, tree.nodes[root].color);
    }

    #[test]
    fn no_child_removal(){
        let mut tree = RedBlackTree{
            nodes: Vec::new()
        };
        let mut root = tree.insert(Node::new(13), None);
        println!("root: {:?}, color: {:?}", root, tree.nodes[root].color);
        root = tree.insert(Node::new(14), Some(root));
        root = tree.insert(Node::new(11), Some(root));
        tree.remove(2);
        
    }
} 


