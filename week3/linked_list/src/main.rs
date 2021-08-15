use linked_list::LinkedList;
pub mod linked_list;

fn main() {
    let mut list: LinkedList<u32> = LinkedList::new();
    assert!(list.is_empty());
    assert_eq!(list.get_size(), 0);
    for i in 1..12 {
        list.push_front(i);
    }

    let mut lc = list.clone();

    println!("lc equal {} list ", lc.eq(&list));
    println!("{}", list);
    println!("list size: {}", list.get_size());
    println!("top element: {}", list.pop_front().unwrap());
    println!("{}", list);
    println!("size: {}", list.get_size());
    println!("{}", list.to_string()); // ToString impl for anything impl Display
    

    println!("lc equal {} list ", lc.eq(&list));
    
    println!("lc {}", lc);
    println!("lc list size: {}", lc.get_size());
    println!("lc top element: {}", lc.pop_front().unwrap());
    println!("lc {}", lc);
    println!("lc size: {}", lc.get_size());
    println!("lc {}", lc.to_string()); // ToString impl for anything impl Display
    // If you implement iterator trait:
    //for val in &list {
    //    println!("{}", val);
    //}

    println!("lc equal {} list ", lc.eq(&list));
}
