use std::collections::HashMap;
//use rand::{thread_rng, Rng};

// Standard library enum: Option
fn main() {
    let ints: Vec<u8> = vec![2, 9, 2, 5, 6, 2, 4, 5, 8, 2, 4, 5, 6, 3, 2, 8, 2, 2, 4];
    //thread_rng().fill(&mut ints[..]);
    getmedian_mode(&ints);
    /*
    let mut v= vec![120,240];
    for i in &mut v{
        println!("{}",*i+100);
    }
    let mut s1 = "LoL".to_string();
    s1.push_str("zzzz");
    let mut s2 = String::from("d");
    s2.push('x');
    let s3 = s1 + &s2; //deref coercion turns &s2 --> &s2[..]
    println!("{}", s3);

    let s4 = "tic".to_string();
    let s5 = String::from("tac");
    let s6 = String::from("toe");
    let s7 = format!("{}-{}-{}",s4,s5,s6);
    println!("{}",s7);

    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 200);
    scores.insert(String::from("red"), 500);
    scores.insert(String::from("violet"), 7700);

    let k = String::from("blue");
    // Here, score will have the value that’s associated with the Blue team, and the result will be 10.
    // The get method returns an Option<&V>; if there’s no value for that key in the hash map, get will return None.
    // This program handles the Option by calling copied to get an Option<i32> rather than an Option<&i32>,
    // then unwrap_or to set score to zero if scores doesn't have an entry for the key.
    let sco = scores.get(&k).copied().unwrap_or(0);

    //if !containsKey() -> insert
    scores.entry("Red".to_string()).or_insert(250);


    let mut worldMap = HashMap::new();
    let text = "What a wonderful world to live in this world again world".to_string();
    for word in text.split_whitespace(){
        // First time world is encountered; it will be initiated
        let cnt = worldMap.entry(word).or_insert(0);
        *cnt+=1;
    }
    println!("{:?}", worldMap);

    */
}

fn getmedian_mode(arr: &[u8]) {
    let mut ele_by_cnt_map = HashMap::new();
    let mut mode_val: u8 = 0;
    let mut sorted_arr = arr.to_vec();
    sorted_arr.sort();
    for ele in arr {
        // First time element is encountered
        let cnt = ele_by_cnt_map.entry(ele).or_insert(0);
        *cnt += 1;
        mode_val = if cnt >= &mut mode_val { *cnt } else { mode_val };
    }
    println!("{:#?}", ele_by_cnt_map);
    println!("Modal {:?}", mode_val);
    println!(" Sorted Array {:?}", sorted_arr);
    let median:f64 = {
        let midpt = sorted_arr.len() % 2;
        let mut med:f64 = 0.0;
        if midpt%2==0{
            med=sorted_arr[midpt].into();
        }else{
            med=((sorted_arr[midpt+1]+sorted_arr[midpt])/2).into();
        }
        med
    };
    println!(" Median {:?}", median);
    println!(" Array size {:?}", sorted_arr.len());
}
