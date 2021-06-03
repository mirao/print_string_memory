// Print chars of string, use memory address
// https://pramode.in/2016/09/13/using-unsafe-tricks-in-rust/

fn print_chars(p: &String) {
    println!("&str: {:p}", p);
    unsafe {
        let addr_inner = *(p as *const String as *const usize);
        println!("str: {:#x}", addr_inner);
        // Print string char by char
        for i in 0..p.len() {
            let hex = *((addr_inner + i) as *const u8) as char;
            print!("{}", hex);
        }
    }
    println!();
}

fn main() {
    let name = String::from("Rust is a language");
    print_chars(&name);
}
