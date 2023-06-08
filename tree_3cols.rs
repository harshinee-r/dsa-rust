use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct TreeNode {
    pub name: String,
    pub salary: f64,
    pub children: Vec<Rc<RefCell<TreeNode>>>,
    pub rm: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new() -> TreeNode {
        return TreeNode {
            name: String::new(),
            salary: 0.0,
            children: Vec::new(),
            rm: None,
        };
    }
}

pub fn printt(s: &Rc<RefCell<TreeNode>>) {
    let s = s.borrow_mut();
    match &s.rm {
        Some(val) => println!("{:10}|{:10}|{:10}", s.name, val.borrow_mut().name, s.salary),
        None => println!(" "),
    };
    for i in s.children.iter() {
        //let mut i=i.borrow_mut();
        printt(i);
    }
}
pub fn deletet(s: &Rc<RefCell<TreeNode>>) {
    let s = &(s.borrow_mut());
    for i in s.children.iter() {
        deletet(i)
    }
    //s.name="".to_string();
    //s.children=Vec::new();
    //s.salary=0.00;
    //s.rm=None;
    drop(s);
}

pub fn insert_row(arr: (String, String, f64), rt: Rc<RefCell<TreeNode>>) -> Rc<RefCell<TreeNode>> {
    let node = Rc::new(RefCell::new(TreeNode::new()));
    let current = Rc::clone(&node);
    let mut current = current.borrow_mut();
    current.name = arr.0;
    current.salary = arr.2;
    let r = rt.clone();
    let (a, b) = search(r, &arr.1);
    if a {
        let b_ = b.clone();
        current.rm = b;
        match b_ {
            Some(val) => val.borrow_mut().children.push(node),
            None => println!(" "),
        }
        //b.children.push(current);
        return rt;
    } else {
        let node_ = Rc::new(RefCell::new(TreeNode::new()));
        let root = Rc::clone(&node_);
        //let mut root=root.borrow_mut();
        let root__ = root.clone();
        let root2 = root.clone();
        current.rm = Some(root__);
        let mut root_ = root.borrow_mut();
        root_.name = arr.1;
        root_.children.push(node);
        return root2;
    }
}
fn search(root: Rc<RefCell<TreeNode>>, rm: &String) -> (bool, Option<Rc<RefCell<TreeNode>>>) {
    let root__ = Rc::clone(&root);
    let root_ = root.borrow_mut();

    if root_.name == *rm {
        return (true, Some(root__));
    }
    for i in root_.children.clone().into_iter() {
        //let i = i.borrow_mut();
        let i_ = Rc::clone(&i);
        return search(i_, rm);
    }

    return (false, None);
}

pub fn add_row(arr: (String, String, f64), rt: Rc<RefCell<TreeNode>>) -> Rc<RefCell<TreeNode>> {
    let root = insert_row(arr, rt);
    let root1 = root.clone();
    printt(&root);
    return root1;
}
