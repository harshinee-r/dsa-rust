use std::cell::RefCell;
use std::rc::Rc;

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
    let s = s.borrow();
    if s.name==""&& s.salary==0.00{
        return;
    }
    match &s.rm {
        Some(val) => println!("{:25}|{:25}|{:25}", s.name, val.borrow().name, s.salary),
        None => println!("{:25}|{:25}|{:25}", s.name," ".to_string(), s.salary),
    };
    for i in s.children.iter() {
        //let mut i=i.borrow_mut();
        printt(i);
    }
}
pub fn printn(s:&Rc<RefCell<TreeNode>>){
    let s=s.borrow();
    print!("{} :",s.name);
    for i in s.children.clone().iter(){
        let i=i.borrow();
        print!("{}, ",i.name);
    }
    print!("\n");
    for i in s.children.clone().iter(){
        printn(i);
    }
}
pub fn deletet(s: &Rc<RefCell<TreeNode>>) {
    let mut s = s.borrow_mut();
    if s.children.len()==0
    {
        return;
    }
    for i in s.children.iter() {
        deletet(i)
    }
    s.name="".to_string();
    s.children=Vec::new();
    s.salary=0.00;
    s.rm=None;
    
}

pub fn insert_row(arr: (String, String, f64), rt: Rc<RefCell<TreeNode>>) -> Rc<RefCell<TreeNode>> {
    if rt.borrow().name==arr.0 {
        rt.borrow_mut().salary=arr.2;
        let root=Rc::new(RefCell::new(TreeNode::new()));
        let root_=Rc::clone(&root);
        let root__=Rc::clone(&root);
        rt.borrow_mut().rm=Some(root__);
        let mut root_=root_.borrow_mut();
        root_.name=arr.1;
        root_.children.push(rt);
        return root;
    }
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
            None => println!(""),
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
    //println!("{:25}|{:25}|{:25}", " Employee Name".to_string(),"Reporting Manager Name".to_string(), "Salary".to_string());
    // printt(&root);
    print!("\n");
    // printn(root);
    return root1;
}
fn c (s:&Rc<RefCell<TreeNode>>, co:i64)->i64{
    let s=s.borrow();
    let mut co=co;
    for i in s.children.clone().iter(){
        let k= c(i,co);
        if k>co
        {
            co=k;
        }
    }
    co=co+1;
    return co;
}

pub fn printtree (s: &Rc<RefCell<TreeNode>>)
{
let cou= c(s,0);
let width=(cou*7) as usize;
print!("{:>width$}",s.borrow().name);
printtree_h(s,cou);
}
fn printtree_h(s:&Rc<RefCell<TreeNode>>,co:i64)
{   let s=s.borrow();
    let mut co=co;
    //co=co-1;
    let mut width=((co-1)*7) as usize +s.name.len();
    for i in 0..s.children.len(){
        let k=s.children[i].borrow();
        if i==0{
            width=width-(s.name.len()-1) as usize;
         print!("\n{:>width$}","/");
         if s.children.len()>1
          {let width=s.name.len()-1;
            let st=r"\";
            print!("{:width$}{}"," ",st);
          }
         print!("\n{:>width$}",k.name);
        }
        else {
            //let st=r"\";
            let width=s.name.len();
            print!("{:width$}{}"," ",k.name);
        }
        
    }
    co=co-1;
    for i in s.children.iter(){
        printtree_h(i,co)
    }
}
