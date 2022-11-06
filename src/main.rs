
fn main() {
    let x: (i32,f64,u8,char) = (500,6.3,3,'s');
    println!("{:?}",x);
    let five_hundo=x.0;
    let s_char=x.3;
    println!("{five_hundo} {s_char}");

    const A: [i32; 5] = [1,2,3,4,5];
    println!("{:#?}", A);

    let t = {
        let x = 3;
        x+1
    };
    println!("value of T {t}");
    let mut count = 0;
    let result = loop {
        count += 1;
        if count == 10{
            break count*5;
        }
        println!("Count now {count}");
    };
    println!("Count Res {result}");

    let mut counta = 0;
    'counting_up: loop {
        println!("counta = {counta}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if counta == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        counta += 1;

        
    }
    println!("End count = {counta}");
    for number in (1..4).rev() {
        println!("{number}!");
    }
}
