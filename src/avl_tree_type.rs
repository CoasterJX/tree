use super::*;
use std::fmt::Debug;
use avl_tree::{AVLTreeNode as AVL, Direction as Dir, AVLChild as TRoot};

#[derive(Clone, Debug)]
pub struct AVLTree<T: Ord + Clone> {
    pub root: TRoot<T>,
    pub num_leaves: u128,
    pub height: u128,
    _is_num_leaves_available: bool,
    _is_height_available: bool,
}

impl<T: Ord + Clone + Debug> AVLTree<T> {

    pub fn new() -> AVLTree<T> {
        AVLTree { 
            root: None, 
            num_leaves: 0, 
            height: 0,
            _is_num_leaves_available: false,
            _is_height_available: false
        }
    }

    pub fn get_num_leaves(&mut self) -> u128 {
        if self._is_num_leaves_available {
            return self.num_leaves;
        }
        self.num_leaves = AVL::count_leaves(&self.root);
        self._is_num_leaves_available = true;
        return self.num_leaves;
    }

    pub fn get_height(&mut self) -> u128 {
        if self._is_height_available {
            return self.height;
        }
        self.height = AVL::get_height(&self.root);
        self._is_height_available = true;
        return self.height;
    }

    pub fn print_tree(&self) {
        AVL::print_tree(&self.root)
    }

    pub fn transplant(&mut self, u: &TRoot<T>, v: &TRoot<T>) {
        /*
            This function effectively removes the node pointed to at u and replaces it with the node pointed to at v.
        */
        let u_node = u.clone();
        let v_node = v.clone();
        match AVL::get_parent(&u_node) {
            Some(_) => {
                if AVL::is_node_equal(&u_node, &AVL::get_left(&AVL::get_parent(&u_node))) {  // If the u_node is a left child.
                    AVL::set_child(&AVL::get_parent(&u_node), v_node.clone(), Dir::Left);  // Set the left child of u's parent
                    // to be v. In other words, removes u and replaces it with v.
                } else {  // u_node is a right child
                    AVL::set_child(&AVL::get_parent(&u_node), v_node.clone(), Dir::Right);  // Replaces u with v
                }
            },
            None => self.root = v_node.clone(),
        };
        AVL::set_parent(&v_node, &AVL::get_parent(&u_node));  // Set v's parent to be the parent of u.
    }

    pub fn delete_fixup(&mut self, key: T) {
        /*
            Once a node in the AVL tree got deleted the tree needs to be fixed. This is where tree rotations will be called
            to ensure the AVL tree properties are satisfied.
        */
        let mut z = AVL::find_node(&self.root, key.clone());
        loop {  // Infinite loop. Nothing more to say here.
            z = AVL::get_parent(&z);  // We want to perform the tree rotations at the parent.

            let bf: i64 = AVL::get_balance_factor(&z);  // This is the balance factor. This is a measure of how
            // balanced or unbalanced the AVL tree is.

            /*
                When the AVL tree is unbalance there are four cases to consider: left-left, left-right, right-right,
                and right-left. Please refer to this video for more information: https://www.youtube.com/watch?v=1QSYxIKXXP4
            */
            if bf == -2 {  // A bf of -2 means that the current tree is very left heavy
                if AVL::get_balance_factor(&AVL::get_left(&z)) <= 0 {
                    // Left-Left case
                    self.root = AVL::right_rotate(self.root.clone(), AVL::get_root_key(&z));
                    z = AVL::get_parent(&z);  // After the rotation z would be moved down one layer. This set's node z
                    // to be the node that took it's place.
                } else {
                    // Left-Right case
                    self.root = AVL::left_rotate(self.root.clone(), AVL::get_root_key(&AVL::get_left(&z)));
                    self.root = AVL::right_rotate(self.root.clone(), AVL::get_root_key(&z));
                    z = AVL::get_parent(&z);  // After the rotation z would be moved down one layer. This set's node z
                    // to be the node that took it's place.
                }
            } else if bf == 2 {
                if AVL::get_balance_factor(&AVL::get_right(&z)) >= 0 {
                    // Right-Right case
                    self.root = AVL::left_rotate(self.root.clone(), AVL::get_root_key(&z));
                    z = AVL::get_parent(&z);  // After the rotation z would be moved down one layer. This set's node z
                    // to be the node that took it's place.
                } else {
                    // Right-Left case
                    self.root = AVL::right_rotate(self.root.clone(), AVL::get_root_key(&AVL::get_right(&z)));
                    self.root = AVL::left_rotate(self.root.clone(), AVL::get_root_key(&z));
                    z = AVL::get_parent(&z);  // After the rotation z would be moved down one layer. This set's node z
                    // to be the node that took it's place.
                }
            }
            if AVL::is_node_equal(&None, &z) {  // This part checks if node z is the root node. In this case we
            // want to break ouf the infinite loop. If not we need to continue to perform tree rotations to fix
            // the tree.
                break;
            }
        }
    }

    pub fn delete(&mut self, key: &T) {
        /*
            This function simply removes the node with value of key. This function implements the AVL tree delete
            algorithm by William Fiset shown here: https://www.youtube.com/watch?v=g4y2h70D6Nk
        */
        self._is_num_leaves_available = false;
        self._is_height_available = false;
        let z = AVL::find_node(&self.root, key.clone());
        match z {
            None => return,
            _ => (),
        };
        // Special case when the only node in the AVL tree is the root and we want to delete the root.
        if AVL::is_node_equal(&self.root, &z)
        && AVL::get_root_nil(&AVL::get_left(&self.root))
        && AVL::get_root_nil(&AVL::get_right(&self.root)) {
            self.root = None;
            return;
        }

        if AVL::get_root_nil(&AVL::get_left(&z)) {  // Left subtree of z is NIL
            if AVL::get_root_nil(&AVL::get_right(&z)) {  // Right subtree of z is NIL
                /*
                    Node to remove is a leaf node.
                */
                let u_node = z.clone();
                match AVL::get_parent(&u_node) {
                    Some(_) => {
                        if AVL::is_node_equal(&u_node, &AVL::get_left(&AVL::get_parent(&u_node))) {  // If the u_node is a left child.
                            AVL::set_child_nil(&AVL::get_parent(&u_node), Dir::Left);  // Set the left child of u's parent
                            // to be NIL. In other words, removes u.
                        } else {  // u_node is a right child
                            AVL::set_child_nil(&AVL::get_parent(&u_node), Dir::Right);  // Replaces u with NIL
                        }
                    },
                    None => {},
                };
            } else {  // Right subtree of z is not NIL
                /*
                    Node to remove has a right subtree but no left subtree.
                */
                let u_node = z.clone();
                let v_node = AVL::get_right(&z);
                self.transplant(&u_node, &v_node);
                AVL::virtualize_all_nil(&self.root);
                self.delete_fixup(AVL::get_root_key(&v_node));
            }
        } else {  // Left subtree of z is not NIL
            if AVL::get_root_nil(&AVL::get_right(&z)) {  // Right subtree of z is NIL
                /*
                    Node to remove has a left subtree but no right subtree.
                */
                let u_node = z.clone();
                let v_node = AVL::get_left(&z);
                self.transplant(&u_node, &v_node);
                AVL::virtualize_all_nil(&self.root);
                self.delete_fixup(AVL::get_root_key(&v_node));
            } else {  // Right subtree of z is not NIL
                /*
                    Node to remove has both a left subtree and a right subtree.
                */
                let min_node = AVL::get_minimum(&AVL::get_right(&z));
                AVL::set_root_key(&z, AVL::get_root_key(&min_node));
                if AVL::get_root_nil(&AVL::get_right(&min_node)) {  // min_node is a leaf node
                    let u_node = min_node.clone();
                    let parent_node = AVL::get_parent(&u_node);
                    match parent_node {
                        Some(_) => {
                            if AVL::is_node_equal(&u_node, &AVL::get_left(&AVL::get_parent(&u_node))) {  // If the u_node is a left child.
                                AVL::set_child_nil(&AVL::get_parent(&u_node), Dir::Left);  // Set the left child of u's parent
                                // to be NIL. In other words, removes u.
                            } else {  // u_node is a right child
                                AVL::set_child_nil(&AVL::get_parent(&u_node), Dir::Right);  // Replaces u with NIL
                            }
                        },
                        None => {},
                    };
                    AVL::virtualize_all_nil(&self.root);
                    self.delete_fixup(AVL::get_root_key(&parent_node));
                } else {  // min_node has a right subtree
                    let u_node = min_node.clone();
                    let v_node = AVL::get_right(&u_node);
                    self.transplant(&u_node, &v_node);
                    AVL::virtualize_all_nil(&self.root);
                    self.delete_fixup(AVL::get_root_key(&v_node));
                }
            }
        }
        //AVL::virtualize_all_nil(&self.root);  // The set_child_nil function doesn't actually remove the node from the AVL tree.
        // Instead it sets the value of the node to be the parent node. It also sets the parent pointer to be itself. In order
        // for the node to be deallocated you need to call the virtualize_all_nil function.
    }

    pub fn insert(&mut self, key: &T) {
        /*
            The match statement below inserts a new node with value of key into the tree. This
            does not perform any tree rotations to keep the AVL tree properties satisfied.
        */
        self._is_num_leaves_available = false;
        self._is_height_available = false;
        match &self.root {
            Some(_) => AVL::insert_node(&self.root, key.clone()),
            None => self.root = AVL::new(key.clone()),
        };
        let mut z = AVL::find_node(&self.root, key.clone());  // Get the node with value of key
        
        loop {  // Infinite loop. Nothing more to say here.
            z = AVL::get_parent(&z);  // We want to perform the tree rotations at the parent.

            let bf: i64 = AVL::get_balance_factor(&z);  // This is the balance factor. This is a measure of how
            // balanced or unbalanced the AVL tree is.

            /*
                When the AVL tree is unbalance there are four cases to consider: left-left, left-right, right-right,
                and right-left. Please refer to this video for more information: https://www.youtube.com/watch?v=1QSYxIKXXP4
            */
            if bf == -2 {  // A bf of -2 means that the current tree is very left heavy
                if AVL::get_balance_factor(&AVL::get_left(&z)) <= 0 {
                    // Left-Left case
                    self.root = AVL::right_rotate(self.root.clone(), AVL::get_root_key(&z));
                    z = AVL::get_parent(&z);  // After the rotation z would be moved down one layer. This set's node z
                    // to be the node that took it's place.
                } else {
                    // Left-Right case
                    self.root = AVL::left_rotate(self.root.clone(), AVL::get_root_key(&AVL::get_left(&z)));
                    self.root = AVL::right_rotate(self.root.clone(), AVL::get_root_key(&z));
                    z = AVL::get_parent(&z);  // After the rotation z would be moved down one layer. This set's node z
                    // to be the node that took it's place.
                }
            } else if bf == 2 {
                if AVL::get_balance_factor(&AVL::get_right(&z)) >= 0 {
                    // Right-Right case
                    self.root = AVL::left_rotate(self.root.clone(), AVL::get_root_key(&z));
                    z = AVL::get_parent(&z);  // After the rotation z would be moved down one layer. This set's node z
                    // to be the node that took it's place.
                } else {
                    // Right-Left case
                    self.root = AVL::right_rotate(self.root.clone(), AVL::get_root_key(&AVL::get_right(&z)));
                    self.root = AVL::left_rotate(self.root.clone(), AVL::get_root_key(&z));
                    z = AVL::get_parent(&z);  // After the rotation z would be moved down one layer. This set's node z
                    // to be the node that took it's place.
                }
            }
            if AVL::is_node_equal(&None, &z) {  // This part checks if node z is the root node. In this case we
            // want to break ouf the infinite loop. If not we need to continue to perform tree rotations to fix
            // the tree.
                break;
            }
        }
    }
}