use std::io;
mod binary_search;
mod fibonacci;
mod merge_sorting;
mod tree_3cols;
use crate::tree_3cols::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;
fn main() {
    println!("enter bs for binary search of an element in the stored array");
    println!("enter fib for getting the nth fibonacci number");
    println!("enter ms for performing merge sort on the user inputed array");
    println!("enter tt for entering tree table");
    let mut ans = String::new();
    io::stdin()
        .read_line(&mut ans)
        .expect("not a valid answer!");
    let ans = ans.trim();
    //binary search code begins
    if ans == "bs".to_string() {
        let arr: [i32; 5] = [4, 6, 8, 10, 12];
        println!("enter the item to be searched in the stored array");
        let mut item = String::new();
        io::stdin()
            .read_line(&mut item)
            .expect("not a valid number!");
        let item: i32 = item.trim().parse().expect(" ");
        binary_search::b_search(item, &arr);
    } else if ans == "fib".to_string() {
        println!("enter the value of n to print nth fibonocii number");
        let mut n = String::new();
        io::stdin().read_line(&mut n).expect("enter a valid number");
        let mut n: i32 = n.trim().parse().expect(" ");
        fibonacci::fibon(&mut n);
    } else if ans == "ms" {
        let mut arr: [i32; 6] = [7, 5, 2, 3, 1, 4];
        merge_sorting::ms(&mut arr);
        for i in arr.iter() {
            println!("{i}");
        }
    } else if ans == "tt" {
        //let mut arr: (String, String, f64) = (String::new(), String::new(), 0.00);
        //println!("enter the name of the employee");
        //io::stdin()
          //  .read_line(&mut arr.0)
            //.expect("not a valid answer!");
        //arr.0 = arr.0.trim().to_string();
        //println!("enter the name of the reporting manager");
        //io::stdin()
          //  .read_line(&mut arr.1)
           // .expect("not a valid answer!");
       // arr.1 = arr.1.trim().to_string();
        //let mut n = String::new();
        //println!("enter the salary of the employee");
        //io::stdin().read_line(&mut n).expect("not a valid answer!");
        //let n: f64 = n.trim().parse().expect(" ");
        //arr.2 = n;
        let rt = Rc::new(RefCell::new(TreeNode::new()));
        let mut root = Rc::clone(&rt);
        root.borrow_mut();
        //root = tree_3cols::add_row(arr, root);
        let arr1:(String,String,f64)=("ramya".to_string(), "shankar".to_string(), 60000.00);
        let arr2: (String, String, f64) = ("david".to_string(), "ramya".to_string(), 32000.00);
        let arr3: (String, String, f64) = ("rajesh".to_string(), "shankar".to_string(), 60000.00);
        let arr4: (String, String, f64) = ("shankar".to_string(), "siva".to_string(), 100000.00);
        root = tree_3cols::add_row(arr1, root);
        root = tree_3cols::add_row(arr2, root);
        root= tree_3cols::add_row(arr3, root);
        root= tree_3cols::add_row(arr4, root);
        println!("{:25}|{:25}|{:25}", " Employee Name".to_string(),"Reporting Manager Name".to_string(), "Salary".to_string());
        tree_3cols::printt(&root);
        println!("");
        tree_3cols::printn(&root);
        println!("");
        tree_3cols::printtree(&root);
        print!("\n\n");
        tree_3cols::deletet(&root);
        //println!("{}&{}",t.name,s.children.len());
        println!("deleted the tree");
        tree_3cols::printt(&root);
    } else {
        println!("invalid entry");
        //practice
    }
}
