pub mod tree_test;
pub mod red_black_tree;
pub mod tree_type;

use tree_type::RedBlackTree as RBT;
use std::io::{self, Write};
use std::fmt::Debug;
use std::str::FromStr;

const LINE: &str = "-----------------------------------------------------";

fn welcome() {
    println!("{}", LINE);
    log("ECE 421 Project 2 - Red Black Tree & AVL Tree");
    log("Jianxi Wang, Yihe Wang, John Yu");
    println!("{}", LINE);
}

fn invalid(cmd: &Vec<&str>, size: usize) -> bool {
    if cmd.len() < size {
        println!("Invalid command: {}", cmd.join(" "));
    }
    return cmd.len() < size
}

fn exec_cmd<T: Ord + Clone + Debug + FromStr>(tree: &mut RBT<T>, cmd: &str) where <T as FromStr>::Err: Debug {
    let c = cmd.trim().split(" ").collect::<Vec<&str>>();
    match c[0] {
        "insert" => {
            if invalid(&c, 2) {return;}
            for i in 1..c.len() {
                let key = c[i].parse().unwrap();
                tree.insert(&key);
            }
        },
        "delete" => {
            if invalid(&c, 2) {return;}
            for i in 1..c.len() {
                let key = c[i].parse().unwrap();
                tree.delete(&key);
            }
        },
        "count_leaves" => {
            if invalid(&c, 1) {return;}
            println!("{:?}", tree.get_num_leaves());
        },
        "height" => {
            if invalid(&c, 1) {return;}
            println!("{:?}", tree.get_height());
        },
        "is_empty" => {
            if invalid(&c, 1) {return;}
            println!("{:?}", tree.is_empty());
        },
        "print" => {
            if invalid(&c, 1) {return;}
            tree.print_tree();
        },
        "traverse" => {
            if invalid(&c, 2) {return;}
            match c[1] {
                "asc" => tree.print_traverse(std::cmp::Ordering::Less),
                "desc" => tree.print_traverse(std::cmp::Ordering::Greater),
                _ => println!("Invalid traverse option."),
            };
        },
        _ => println!("Invalid command: {}", cmd),
    }
}

fn log(str: &str) {
    println!("| {:?}", str);
}

fn start_demo<T: Ord + Clone + Debug + FromStr>(rbt: &mut RBT<T>) where <T as FromStr>::Err: Debug {
    loop {
        print!(">>> ");
        io::stdout().flush().unwrap();
        let mut line = String::new();
        let _ = io::stdin().read_line(&mut line).unwrap();
        exec_cmd(rbt, &line);
    }
}

fn main() {
    welcome();
    let mut key_type = String::new(); 
    print!("Decide the type of tree key: ");
    io::stdout().flush().unwrap();
    let _ = io::stdin().read_line(&mut key_type).unwrap();
    
    match key_type.trim() {
        "int" => {
            let mut rbt: RBT<i128> = RBT::new();
            start_demo(&mut rbt);
        },
        "str" => {
            let mut rbt: RBT<String> = RBT::new();
            start_demo(&mut rbt);
        }
        _ => println!("Type not implemented in this demo but should work in real case.")
    }
}
