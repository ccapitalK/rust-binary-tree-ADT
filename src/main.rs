use std::cmp::Ordering;
use std::cmp::max;
use std::io::{self,Write};
use std::fmt::Debug;

#[derive(Clone,Debug)]
enum BST<T> where T: Ord {
    INode(Box<BST<T>>,T,Box<BST<T>>),
    Leaf(T),
    None,
}

fn leaves<T: Ord + Debug>(bt : &BST<T>) -> usize {
    match *bt {
        BST::INode(ref l,_,ref r) => leaves(l) + leaves(r),
        BST::Leaf(_) => 1,
        BST::None => 0,
    }
}

fn edges<T: Ord + Debug>(bt : &BST<T>) -> usize {
    match *bt {
        BST::INode(ref l,_,ref r) => {
            (match **l { BST::None => 0, _ => edges(l) + 1}) +
            (match **r { BST::None => 0, _ => edges(r) + 1})
        },
        _ => 0,
    }
}

fn height<T: Ord + Debug>(bt : &BST<T>) -> usize {
    match *bt {
        BST::INode(ref l,_,ref r) => 1 + max(height(l), height(r)),
        BST::Leaf(_) => 1,
        BST::None => 0,
    }
}

fn nodes<T: Ord + Debug>(bt : &BST<T>) -> usize {
    match *bt {
        BST::INode(ref l,_,ref r) => 1 + nodes(l) + nodes(r),
        BST::Leaf(_) => 1,
        BST::None => 0,
    }
}

fn repl_help_string(){
    println!("Commands:");
    println!("    a[dd] <val>:    add val to tree");
    println!("    in <val>:       prints whether val is in the tree");
    println!("    d[el] <val>:    delete val from tree");
    println!("    i[nfo]:         print info about the tree");
    println!("    p[rint]:        print the tree");
    println!("    q[uit]:         quit this emulator");
}

fn main() {
    let mut bt : BST<i32> = BST::None;
    //TODO: Create REPL for this 
    'main: loop {

        print!("> ");
        io::stdout().flush().expect("Failed to flush output buffer");

        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line from stdin");

        let mut words = input.split_whitespace();

        let get_num = |next_word: Option<&str>| {
            match next_word {
                Some(v) => match v.parse() {
                    Ok(v) => Some(v),
                    Err(e) => {
                        println!("Failed to parse integer: {}", e);
                        None
                    }
                },
                None => {
                    repl_help_string();
                    None
                }
            }
        };

        if let Some(word) = words.next() {
            match word {
                "a"|"add" => {
                    if let Some(v) = get_num(words.next()) {
                        bt.insert(v);
                    }
                },
                "d"|"del" => {
                    if let Some(v) = get_num(words.next()) {
                        bt.delete(v);
                    }
                },
                "i"|"info" => {
                    println!("Nodes: {}", nodes(&bt));
                    println!("Leaves: {}", leaves(&bt));
                    println!("Edges: {}", edges(&bt));
                    println!("Height: {}", height(&bt));
                },
                "in" => {
                    if let Some(v) = get_num(words.next()) {
                        if bt.contains(v) {
                            println!("Yes");
                        } else {
                            println!("No");
                        }
                    }
                },
                "p"|"print" => {
                    println!("{:?}", bt);
                },
                "q"|"quit" => {
                    break 'main;
                },
                _ => repl_help_string(),
            }
        } else {
            repl_help_string();
        }

    }
}

trait Bag<T> {
    fn contains(&self, val: T) -> bool;
    fn delete(&mut self, val: T);
    fn insert(&mut self, val: T);
    fn min_element(&self) -> Option<T>;
    fn max_element(&self) -> Option<T>;
}

impl<T> Bag<T> for BST<T> where T: Ord + Clone + Debug {
    fn insert(&mut self, val: T) {
        let update = match *self {
            BST::INode(ref mut l, ref d, ref mut r) => {
                match d.cmp(&val) {
                    Ordering::Less => {r.insert(val)},
                    Ordering::Greater => {l.insert(val)},
                    _ => {}
                };
                None
            },
            BST::Leaf(ref d) => {
                if *d != val {
                    let mut new = BST::INode(Box::new(BST::None), d.clone(), Box::new(BST::None));
                    new.insert(val);
                    Some(new)
                } else {
                    None
                }
            },
            BST::None => Some(BST::Leaf(val)),
        };
        if let Some(v) = update { 
            *self = v;
        }
    }
    fn delete(&mut self, val: T){
        let new_node = match *self {
            BST::INode(ref mut l, ref mut d, ref mut r) => match val.cmp(d) {
                Ordering::Less    => {
                    l.delete(val);
                    None
                },
                Ordering::Equal   => {
                    match l.max_element() {
                        None => match r.min_element() {
                            Some(v) => {
                                if nodes(r) > 1 {
                                    r.delete(v.clone());
                                    *d = v.clone();
                                    None
                                } else {
                                    Some(BST::Leaf(v.clone()))
                                }
                            }
                            None => panic!("BST contains node of type INode(None, _, None) (Broken DS Invariant)"),
                        },
                        Some(v) => {
                            l.delete(v.clone());
                            *d = v.clone();
                            None
                        },
                    }
                },
                Ordering::Greater => {
                    r.delete(val);
                    None
               },
            },
            BST::Leaf(ref v) => if *v == val {
                Some(BST::None)
            }else{
                None
            },
            BST::None    => None,
        };
        if let Some(new_node) = new_node {
            *self = new_node;
        }
    }
    fn contains(&self, val: T) -> bool {
        match *self {
            BST::INode(ref l, ref d, ref r) => {
                match d.cmp(&val) {
                    Ordering::Less => r.contains(val),
                    Ordering::Equal => true,
                    Ordering::Greater => l.contains(val),
                }
            },
            BST::Leaf(ref d) => val == *d,
            BST::None => false,
        }
    }
    fn min_element(&self) -> Option<T> {
        match *self {
            BST::INode(ref l, ref d, _) => {
                if let BST::None = **l {
                    Some(d.clone())
                } else {
                    l.max_element()
                }
            },
            BST::Leaf(ref d) => Some(d.clone()),
            BST::None => None,
        }
    }
    fn max_element(&self) -> Option<T> {
        match *self {
            BST::INode(_, ref d, ref r) => {
                if let BST::None = **r {
                    Some(d.clone())
                } else {
                    r.max_element()
                }
            },
            BST::Leaf(ref d) => Some(d.clone()),
            BST::None => None,
        }
    }
}
