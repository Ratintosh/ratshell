use crate::utils::log::{err, usage};

fn bubble(args:Vec<&str>) {
    let mut set: Vec<i32> = args
        .iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    
    for right in 0..set.len(){
        for left in 0..set.len() - 1 - right {
            if set[left] > set[left + 1] {
                set.swap(left, left+1);
            }
        }
    }

    println!("{:?}", set);


}

pub fn main(args:Vec<&str>) {
    if args.is_empty() {
        err("No flags specified. Aborting.");
        usage("sort [ flags ] [ numbers ]");
        println!("  Flags:");
        println!("      -b      Bubble Sort");
        return
    }

    if args[0] == "-b" {
        if args[1..].is_empty() {
            err("Please input a set of numbers.")
        }
        bubble((&args[1..]).to_vec());
    }

}