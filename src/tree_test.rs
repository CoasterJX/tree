use super::*;
use red_black_tree::{RBTreeNode as RB, NodeColor};
use tree_type::RedBlackTree as RBT;

#[test]
fn test_rb_rotate() {
    let root = RB::new(5);
    for i in [5, 2, 10, 8, 6, 9, 12] {
        RB::insert_node(&root, i);
    }
    RB::print_tree(&root);
    
    // perform right rotate at 10
    let root = RB::left_rotate(root, 12);
    RB::print_tree(&root);
}


#[test]
fn test_rb_insert() {
    let mut rbt = RBT::<u64>::new();
    for i in [5, 2, 10, 8, 6, 9, 12, 13] {
        rbt.insert(&i);
    }
    rbt.print();
}

#[test]
fn test_rb_delete1() {
    let mut rbt = RBT::<u64>::new();
    for i in [12, 8, 15, 5, 9, 13, 19, 10, 23] {
        rbt.insert(&i);
    }
    rbt.delete(&15);
    rbt.print();
}

#[test]
fn test_rb_delete2() {
    let mut rbt = RBT::<u64>::new();
    for i in [12, 8, 15, 5, 9, 13, 23, 1, 10] {
        rbt.insert(&i);
    }
    rbt.delete(&5);
    rbt.print();
}

#[test]
fn test_rb_delete3() {
    let mut rbt = RBT::<u64>::new();
    for i in [12, 8, 15, 1, 9, 13, 23, 10] {
        rbt.insert(&i);
    }
    rbt.delete(&12);
    rbt.print();
}

#[test]
fn test_rb_transplant1() {
    let mut rbt = RBT::<u64>::new();
    for i in [15, 12, 19, 9, 13, 23] {
        rbt.insert(&i);
    }
    rbt.transplant(
        &RB::find_node(&rbt.root, 15),
        &RB::find_node(&rbt.root, 19)
    );
    rbt.print();
    //RB::print_tree(&rbt.get_minimum());
}

#[test]
fn test_rb_transplant2() {
    let mut rbt = RBT::<u64>::new();
    for i in [15, 12, 19, 13, 23] {
        rbt.insert(&i);
    }
    rbt.transplant(
        &RB::find_node(&rbt.root, 12),
        &RB::find_node(&rbt.root, 13)
    );
    rbt.print();
}

#[test]
fn test_rb_transplant3() {
    let mut rbt = RBT::<u64>::new();
    for i in [15, 12, 19, 8, 23] {
        rbt.insert(&i);
    }
    rbt.transplant(
        &RB::find_node(&rbt.root, 19),
        &RB::find_node(&rbt.root, 23)
    );
    rbt.print();
}

#[test]
fn test_rb_transplant4() {
    let mut rbt = RBT::<u64>::new();
    for i in [15, 12, 19, 8, 23] {
        rbt.insert(&i);
    }
    rbt.transplant(
        &RB::find_node(&rbt.root, 23),
        &None
    );
    rbt.print();
}

// #[test]
// fn test_rb_insert_find() {
//     let root = RBTreeNode::new(5, NodeColor::Black, None);
//     let root_cp = RBTreeNode::find_node(&root, 5);
//     //let root = RBTreeNode::empty();
//     for i in [5, 3, 2, 4, 7, 6, 9, 5] {
//         RBTreeNode::insert_node(&root, i);
//     }
//     // println!("{:#?}", root);
//     //RBTreeNode::print_tree(&root);

//     let root = RBTreeNode::left_rotate(root, 5);
//     RBTreeNode::print_tree(&root);
//     return;
//     RBTreeNode::print_tree(&RBTreeNode::get_parent(&root));
//     println!("{:#?}", RBTreeNode::get_parent(&root));

//     let parent = RBTreeNode::get_parent(&root);
//     RBTreeNode::set_child(&parent, root, red_black_tree::Direction::Right);
//     println!("{:#?}", parent);

//     // let subtree_7 = RBTreeNode::find_node(&root, 7);
//     // //println!("{:#?}", subtree_7);

//     // let nothing_8 = RBTreeNode::find_node(&root, 8);
//     // println!("{:#?}", nothing_8);

//     // let parent_6 = RBTreeNode::get_parent_by_key(&root, 6);
//     // println!("{:#?}", parent_6);

//     // RBTreeNode::set_parent(&subtree_7, &RBTreeNode::find_node(&root, 4));
//     //let new_parent = RBTreeNode::get_parent(&subtree_7);
//     //println!("{:#?}", root);
//     // RBTreeNode::print_tree(&root);
// }

// #[test]
// fn test_rb_set_child() {
//     let root = RBTreeNode::new(2, NodeColor::Black, None);
//     //let root = RBTreeNode::empty();
//     for i in [1, 3] {
//         RBTreeNode::insert_node(&root, i);
//     }

//     let root2 = RBTreeNode::new(2, NodeColor::Black, None);
//     //let root = RBTreeNode::empty();
//     for i in [1, 3] {
//         RBTreeNode::insert_node(&root2, i);
//     }

//     RBTreeNode::set_child(&root, root2, red_black_tree::Direction::Right);
//     println!("{:#?}", root);
// }
