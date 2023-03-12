use super::*;
use red_black_tree::{RBTreeNode as RB, NodeColor};


#[test]
fn test_rb_rotate() {
    let root = RB::new(5);
    for i in [5, 2, 10, 8, 6, 9, 12] {
        RB::insert_node(&root, i);
    }
    RB::print_tree(&root);
    
    // perform left rotate at 10
    let root = RB::left_rotate(root, 10);
    RB::print_tree(&root);
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
