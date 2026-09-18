/*
============================
cowsay command
inputs: anything or nothing
prints: an ascii cow

 ____________
< mooooooooo >
 ------------
        \   ^__^
         \  (oo)\_______
            (__)\       )\/\
                ||----w |
                ||     ||

============================
*/

fn bubble_edge(text: &str, edge: bool) -> String {
    let char = if edge {
        '_'
    } else {
        '-'
    };

    let mut tmp:String = String::new();
    tmp.push(char);
    for _i in text.chars() {
        tmp.push(char);
    }
    tmp.push(char);
    tmp
}

pub fn main(args: Vec<&str>) {
    let text = args.join(" ");
    let bubble_top: String = bubble_edge(&text, true);
    let bubble_bottom: String = bubble_edge(&text, false);
    print!(" {bubble_top}\n< {text} >\n {bubble_bottom}");
    println!(
        r#"
        \   ^__^
         \  (oo)\_______
            (__)\       )\/\
                ||----w |
                ||     ||
        "#);
}