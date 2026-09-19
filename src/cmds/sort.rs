use crate::utils::log::{err, usage};

pub fn main(args:Vec<&str>) {
    if args.is_empty() {
        err("No flags specified. Aborting.");
        usage("sort [ flags ] [ numbers ]");
        println!("  Flags:");
        println!("      -b      Bubble Sort");
        return
    }


}