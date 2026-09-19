use color_print::{cprintln};

pub fn main(args:Vec<&str>) {
    if args.is_empty() {
        cprintln!("<red>[ERROR]</> No flags specified. Aborting.\n");
        cprintln!("<yellow>[USAGE]</> [ -b ] [ numbers ]");
        return
    }
}