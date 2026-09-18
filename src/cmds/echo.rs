/*
============================
echo command
inputs: anything or nothing
prints: the input
============================
*/
pub fn main(args:Vec<&str>) {
    let text = args.join(" ");
    println!("{}", text);
}