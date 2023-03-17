//'cargo run' debug build, added to the target/debug folder
//'cargo run --release' production build, added to the target/release folder
// goes to the target/

fn main() {
    let num: i32 = 1;
    let num_with_auto_type = 1;
    let (even, odd) = (8, 9);
    println!("explicit typed variable. num = {num}");
    println!("auto typed variable. Compiler determines type - numWithAutoType = {num_with_auto_type}");
    println!("multiple variables declared on one line. even = {even}, odd = {odd}");
}
