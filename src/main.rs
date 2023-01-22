use std::thread;
fn main() {
    // Closures vs Functions differences
    println!("------Closures vs Functions---------");

    fn  add_one_v1   (x: u32) -> u32 { x + 1 }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x| { x + 1 };
    let add_one_v4 = |x| x + 1 ;
    // For rustc to correctly infer the type of the closure's argument, the closure must actually be used somewhere.
    println!("{}", add_one_v1(10));
    println!("{}", add_one_v2(20));
    println!("{}", add_one_v3(30));
    println!("{}", add_one_v4(40));
    println!("--------END--------");

    println!("------Immutable borrow---------");

    // Only immutable borrow example for Closure.
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);
    println!("--------END--------");


    println!("------Mutable borrow---------");
    // Mutable borrow example for closure.
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("After calling closure: {:?}", list);
    println!("--------END--------");

    println!("------Threads---------");
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    
    /* 
    Q: Why do we need to use the `move` keyword to pass the ownership to the closure?
    A: In this example, even though the closure body still only needs an immutable reference,
      we need to specify that list should be moved into the closure by putting the move keyword
       at the beginning of the closure definition.
    The new thread might finish before the rest of the main thread finishes, or the main thread might finish first.
    If the main thread maintained ownership of list but ended before the new thread did and dropped list,
      the immutable reference in the thread would be invalid. Therefore, the compiler requires that list be moved
       into the closure given to the new thread so the reference will be valid. 
     */
    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
    println!("--------END--------");


    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{:#?}, sorted in {num_sort_operations} operations", list);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}