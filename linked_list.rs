use std::mem;
pub enum Address{
    Address(Box<Node>),
    Empty,
}
pub struct Node{
    elem:i32,
    next:Address,
}
pub struct Head{
    
    pub head:Address,
}
impl Head{
    pub fn new()->Self{
        return Head{
            head:Address::Empty,
        };
    }
}
pub fn add_val(head:Head,e:i32)->Head{
    let mut head=head;
    let new_node=Box::new(Node {
         elem:e,
         next:mem::replace(&mut head.head,Address::Empty),
        }
    );
    //let mut head=head;
    head.head=Address::Address(new_node);
    return head;
    
}
pub fn print_val(head:&Address){
    match head{
        Address::Empty=>return,
        Address::Address(h)=>{println!("{}",h.elem);print_val(& h.next);}
    }
} 
pub fn pop(s:Head)->Head{
    let mut s=s;
    match s.head{
      Address::Empty=>{s.head=Address::Empty;
        return s;}
      Address::Address(val)=>{
         s.head=val.next;
         return s;
        }
    } 
    
    
         
}
