use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::{Rc, Weak};
use std::cmp::{Ordering, max};

#[derive(Clone, Debug, PartialEq)]
pub enum NodeColor {
    Red,
    Black,
}

impl NodeColor {

    pub fn recolor(&mut self) {
        match &self {
            NodeColor::Red => *self = NodeColor::Black,
            NodeColor::Black => *self = NodeColor::Red,
        }
    }
}

#[derive(PartialEq)]
pub enum Direction {
    Left,
    Right,
}

type TChild<T> = Rc<RefCell<RBTreeNode<T>>>;
type TParent<T> = Weak<RefCell<RBTreeNode<T>>>;
pub type RBTChild<T> = Option<TChild<T>>;
type RBTParent<T> = Option<TParent<T>>;

#[derive(Debug)]
pub struct RBTreeNode<T: Ord + Clone> {
    pub color: NodeColor,
    pub key: T,
    pub parent: RBTParent<T>,
    left_child: RBTChild<T>,
    right_child: RBTChild<T>,
    pub _ptr_self: RBTParent<T>,
    pub is_nil: bool
}

impl<T: Ord + Clone + Debug> RBTreeNode<T> {

    pub fn empty() -> RBTChild<T> {
        None
    }


    pub fn new(key: T) -> RBTChild<T> {
        RBTreeNode::_new(key, NodeColor::Black, None, false)
    }


    pub fn count_leaves(root: &RBTChild<T>) -> u128 {
        if RBTreeNode::get_root_nil(root) {return 0;}
        if RBTreeNode::is_leaf(root) {return 1;}
        RBTreeNode::count_leaves(&RBTreeNode::get_left(root)) + RBTreeNode::count_leaves(&RBTreeNode::get_right(root))
    }


    pub fn get_height(root: &RBTChild<T>) -> u128 {
        if RBTreeNode::get_root_nil(root) {return 0;}
        1 + max(
            RBTreeNode::get_height(&RBTreeNode::get_left(root)),
            RBTreeNode::get_height(&RBTreeNode::get_right(root))
        )
    }


    fn _new(key: T, color: NodeColor, parent: RBTParent<T>, is_nil: bool) -> RBTChild<T> {
        let node = Rc::new(RefCell::new(Self { 
            color, 
            key, 
            parent, 
            left_child: None, 
            right_child: None,
            _ptr_self: None,
            is_nil
        }));
        
        let weak_ptr = Rc::downgrade(&node);
        {
            let mut node_ref = node.borrow_mut();
            node_ref._ptr_self = Some(weak_ptr);
        }
        
        Some(node)
    }


    fn to_string_nil(direction: &Direction, extra: &str, v: &str) {
        let direction_str = match direction {
            Direction::Left => "<──",
            Direction::Right => "──>",
        };
        println!("{}{}NIL{}", extra, direction_str, v);
    }


    fn to_string(color: &NodeColor, direction: &Direction, parent: &RBTChild<T>, key: &T, extra: &str) {
        let color_str = match color {
            NodeColor::Red => "Red",
            NodeColor::Black => "Black",
        };
        let direction_str = match direction {
            Direction::Left => "<──",
            Direction::Right => "──>",
        };
        let parent_key = if let Some(parent_ptr) = parent {
            parent_ptr.borrow().key.clone()
        } else {key.clone()};

        println!("{}", extra);
        println!(
            "{}{}(key {:?}, color {}, parent {:?})", 
            extra, direction_str, key, color_str, parent_key);
        //println!("{}", extra);
    }


    fn _print_tree(root: &RBTChild<T>, direction: Direction, extra: &str) {

        match root {
            Some(tree_ptr) => {
                let node_ref = tree_ptr.borrow();
                RBTreeNode::to_string(
                    &node_ref.color,
                    &direction, 
                    &RBTreeNode::get_parent(&root),
                    &node_ref.key, 
                    extra);
                
                let (left_child, right_child) = (&node_ref.left_child, &node_ref.right_child);
                match left_child {
                    Some(_) => {
                        if RBTreeNode::get_root_nil(left_child) {
                            RBTreeNode::<T>::to_string_nil(&Direction::Left, &(extra.to_owned()+"|\t"), &format!("(solidified, parent {:?})", &node_ref.key));
                        } else {
                            RBTreeNode::_print_tree(left_child, Direction::Left, &(extra.to_owned()+"|\t"));
                        }
                    },
                    None => RBTreeNode::<T>::to_string_nil(&Direction::Left, &(extra.to_owned()+"|\t"), ""),
                };
                match right_child {
                    Some(_) => {
                        if RBTreeNode::get_root_nil(right_child) {
                            RBTreeNode::<T>::to_string_nil(&Direction::Left, &(extra.to_owned()+"|\t"), &format!("(solidified, parent {:?})", &node_ref.key));
                        } else {
                            RBTreeNode::_print_tree(right_child, Direction::Right, &(extra.to_owned()+"|\t"));
                        }
                    },
                    None => RBTreeNode::<T>::to_string_nil(&Direction::Right, &(extra.to_owned()+"|\t"), ""),
                };
            },
            None => RBTreeNode::<T>::to_string_nil(&Direction::Left, "", ""),
        }
    }


    pub fn print_tree(root: &RBTChild<T>) {
        RBTreeNode::_print_tree(root, Direction::Left, "");
    }


    pub fn left_rotate(root: RBTChild<T>, key: T) -> RBTChild<T> {
        let root_copy = root.clone();
        let x = RBTreeNode::find_node(&root, key.clone());
        let parent = RBTreeNode::get_parent(&x);
        match &parent {
            Some(_) => {
                if RBTreeNode::is_node_equal(&x, &RBTreeNode::get_left(&parent)) {
                    RBTreeNode::set_child(
                        &parent, 
                        RBTreeNode::_left_rotate(&root, key), 
                        Direction::Left
                    );
                } else {
                    RBTreeNode::set_child(
                        &parent, 
                        RBTreeNode::_left_rotate(&root, key), 
                        Direction::Right
                    );
                }
                return root_copy;
            },
            None => {
                return RBTreeNode::_left_rotate(&root, key)
            },
        }
    }


    fn _left_rotate(root: &RBTChild<T>, key: T) -> RBTChild<T> {

        match root {
            Some(_) => {

                let x = RBTreeNode::find_node(root, key);
                let y = RBTreeNode::get_right(&x);
                match y {
                    None => return root.clone(),
                    _ => (),
                }
                
                RBTreeNode::set_child(&x, RBTreeNode::get_left(&y), Direction::Right);

                match RBTreeNode::get_left(&y) {
                    Some(_) => RBTreeNode::set_parent(&RBTreeNode::get_left(&y), &x),
                    None => (),
                };
                RBTreeNode::set_parent(&y, &RBTreeNode::get_parent(&x));

                RBTreeNode::set_parent(&x, &y);
                RBTreeNode::set_child(&y, x, Direction::Left);

                return y; // this y must be used to set the parent's left or right

            },
            None => todo!("Does not support empty root now"),
        }
    }

    pub fn right_rotate(root: RBTChild<T>, key: T) -> RBTChild<T> {

        let root_copy = root.clone();
        let x = RBTreeNode::find_node(&root, key.clone());
        let parent = RBTreeNode::get_parent(&x);
        match &parent {
            Some(_) => {
                if RBTreeNode::is_node_equal(&x, &RBTreeNode::get_right(&parent)) {
                    RBTreeNode::set_child(
                        &parent, 
                        RBTreeNode::_right_rotate(&root, key), 
                        Direction::Right
                    );
                } else {
                    RBTreeNode::set_child(
                        &parent, 
                        RBTreeNode::_right_rotate(&root, key), 
                        Direction::Left
                    );
                }
                return root_copy;
            },
            None => {
                return RBTreeNode::_right_rotate(&root, key)
            },
        }
    }

    pub fn _right_rotate(root: &RBTChild<T>, key: T) -> RBTChild<T> {

        match root {
            Some(_) => {

                let x = RBTreeNode::find_node(root, key);
                let y = RBTreeNode::get_left(&x);
                match y {
                    None => return root.clone(),
                    _ => (),
                }
                
                RBTreeNode::set_child(&x, RBTreeNode::get_right(&y), Direction::Left);

                match RBTreeNode::get_right(&y) {
                    Some(_) => RBTreeNode::set_parent(&RBTreeNode::get_right(&y), &x),
                    None => (),
                };
                RBTreeNode::set_parent(&y, &RBTreeNode::get_parent(&x));

                RBTreeNode::set_parent(&x, &y);
                RBTreeNode::set_child(&y, x, Direction::Right);

                return y; // this y must be used to set the parent's left or right

            },
            None => todo!("Does not support empty root now"),
        }
    }


    pub fn is_node_equal(root1: &RBTChild<T>, root2: &RBTChild<T>) -> bool {

        match (root1, root2) {
            (None, None) => todo!("not supported"),
            (Some(ptr1), Some(ptr2)) => {
                return Rc::ptr_eq(ptr1, ptr2);
            },
            _ => false
        }
    }


    pub fn get_color(root: &RBTChild<T>, key: T) -> NodeColor {
        let target = RBTreeNode::find_node(root, key);
        RBTreeNode::get_root_color(&target)
    }


    pub fn get_root_color(root: &RBTChild<T>) -> NodeColor {
        match root {
            Some(target_ptr) => target_ptr.borrow().color.clone(),
            None => NodeColor::Black,
        }
    }


    pub fn get_root_nil(root: &RBTChild<T>) -> bool {
        match root {
            Some(target_ptr) => target_ptr.borrow().is_nil.clone(),
            None => true,
        }
    }


    pub fn set_root_color(root: &RBTChild<T>, color: NodeColor) {
        match root {
            Some(root_ptr) => root_ptr.borrow_mut().color = color,
            None => todo!("should never happen"),
        }
    }


    pub fn get_root_key(root: &RBTChild<T>) -> T {
        match root {
            Some(root_ptr) => root_ptr.borrow().key.clone(),
            None => todo!("should never happen"),
        }
    }


    pub fn get_parent(root: &RBTChild<T>) -> RBTChild<T> {
        match root {
            Some(tree_ptr) => {
                let node_ref = tree_ptr.borrow();
                if let Some(parent_ptr) = node_ref.parent.clone() {
                    return parent_ptr.upgrade();
                } else {return None;}
            }
            None => None,
        }
    }


    pub fn is_leaf(root: &RBTChild<T>) -> bool {
        match root {
            Some(tree_ptr) => {
                let node_ref = tree_ptr.borrow();
                match (&node_ref.left_child, &node_ref.right_child) {
                    (None, None) => return true,
                    _ => return false,
                };
            },
            None => false,
        }
    }


    pub fn get_parent_by_key(root: &RBTChild<T>, key: T) -> RBTChild<T> {
        RBTreeNode::get_parent(&RBTreeNode::find_node(root, key))
    }


    pub fn set_parent(root: &RBTChild<T>, parent: &RBTChild<T>) {
        match root {
            Some(tree_ptr) => {
                let mut node_ref = tree_ptr.borrow_mut();
                match parent {
                    Some(parent_ptr) => node_ref.parent = parent_ptr.borrow()._ptr_self.clone(),
                    None => node_ref.parent = None,
                }
            },
            None => (),
        }
    }


    pub fn set_child(root: &RBTChild<T>, child: RBTChild<T>, direction: Direction) {
        match root {
            Some(tree_ptr) => {
                let mut node_ref = tree_ptr.borrow_mut();
                match direction {
                    Direction::Left => node_ref.left_child = child,
                    Direction::Right => node_ref.right_child = child,
                }
            },
            None => todo!("not supported"),
        }
    }


    pub fn set_child_nil(root: &RBTChild<T>, direction: Direction) {
        match root {
            Some(_) => {
                let nil_node = RBTreeNode::_new(RBTreeNode::get_root_key(root).clone(), NodeColor::Black, None, true);
                match direction {
                    Direction::Left => {
                        RBTreeNode::set_child(root, nil_node, direction);
                        RBTreeNode::set_parent(&RBTreeNode::get_left(root), root);
                    },
                    Direction::Right => {
                        RBTreeNode::set_child(root, nil_node, direction);
                        RBTreeNode::set_parent(&RBTreeNode::get_right(root), root);
                    },
                }
            },
            None => todo!("not supported"),
        }
    }


    pub fn solidify_all_nil(root: &RBTChild<T>) {
        if let None = RBTreeNode::get_left(root) {
            RBTreeNode::set_child_nil(root, Direction::Left);
        } else {
            RBTreeNode::solidify_all_nil(&RBTreeNode::get_left(root));
        };
        if let None = RBTreeNode::get_right(root) {
            RBTreeNode::set_child_nil(root, Direction::Right);
        } else {
            RBTreeNode::solidify_all_nil(&RBTreeNode::get_right(root));
        };
    }


    pub fn virtualize_all_nil(root: &RBTChild<T>) {
        if RBTreeNode::get_root_nil(&RBTreeNode::get_left(root)) {
            RBTreeNode::set_child(root, None, Direction::Left);
        } else {
            RBTreeNode::virtualize_all_nil(&RBTreeNode::get_left(root));
        };
        if RBTreeNode::get_root_nil(&RBTreeNode::get_right(root)) {
            RBTreeNode::set_child(root, None, Direction::Right);
        } else {
            RBTreeNode::virtualize_all_nil(&RBTreeNode::get_right(root));
        };
    }


    pub fn get_left(root: &RBTChild<T>) -> RBTChild<T> {
        match root {
            Some(tree_ptr) => {
                let node_ref = tree_ptr.borrow();
                if let Some(left) = &node_ref.left_child {
                    return Some(left.clone());
                } else {return None;}
            },
            None => None,
        }
    }


    pub fn get_minimum(root: &RBTChild<T>) -> RBTChild<T> {
        match root {
            Some(tree_ptr) => {
                let node_ref = tree_ptr.borrow();
                match &node_ref.left_child {
                    Some(_) => {
                        if RBTreeNode::get_root_nil(&node_ref.left_child) {
                            return root.clone();
                        }
                        return RBTreeNode::get_minimum(&node_ref.left_child)
                    },
                    None => return root.clone(),
                }
            },
            None => todo!("not supported"),
        }
    }


    pub fn get_right(root: &RBTChild<T>) -> RBTChild<T> {
        match root {
            Some(tree_ptr) => {
                let node_ref = tree_ptr.borrow();
                if let Some(left) = &node_ref.right_child {
                    return Some(left.clone());
                } else {return None;}
            },
            None => None,
        }
    }


    pub fn insert_node(root: &RBTChild<T>, key: T) {
        let _ = RBTreeNode::_recurse_node(root, key, true);
    }


    pub fn find_node(root: &RBTChild<T>, key: T) -> RBTChild<T> {
        RBTreeNode::_recurse_node(root, key, false)
    }


    fn _recurse_node(root: &RBTChild<T>, key: T, insert: bool) -> RBTChild<T> {

        match root {
            Some(tree_ptr) => {
                
                let mut node_ref = tree_ptr.borrow_mut();

                match key.cmp(&node_ref.key) {

                    Ordering::Less => {
                        match node_ref.left_child {
                            Some(_) => RBTreeNode::_recurse_node(&node_ref.left_child, key, insert),
                            None => {
                                if insert {
                                    node_ref.left_child = RBTreeNode::_new(key, NodeColor::Red, node_ref._ptr_self.clone(), false);
                                    if let Some(insert_ptr) = &node_ref.left_child {
                                        return Some(insert_ptr.clone());
                                    } else {todo!("should never reach here")}

                                } else {return None;}
                            },
                        }
                    },
                    Ordering::Equal => return Some(tree_ptr.clone()),
                    Ordering::Greater => {
                        match node_ref.right_child {
                            Some(_) => RBTreeNode::_recurse_node(&node_ref.right_child, key, insert),
                            None => {
                                if insert {
                                    node_ref.right_child = RBTreeNode::_new(key, NodeColor::Red, node_ref._ptr_self.clone(), false);
                                    if let Some(insert_ptr) = &node_ref.right_child {
                                        return Some(insert_ptr.clone());
                                    } else {todo!("should never reach here")}
                                } else {return None;}
                            },
                        }
                    },
                }
            },
            None => {return None;},
        }
    }

}