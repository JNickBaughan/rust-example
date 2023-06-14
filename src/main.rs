//'cargo run' debug build, added to the target/debug folder
//'cargo run --release' production build, added to the target/release folder
// goes to the target/

fn main() {


    fn basics() {
        let num: i32 = 1;
        let num_with_auto_type = 1;
        let (even, odd) = (8, 9);
        println!("explicit typed variable. num = {num}");
        println!("auto typed variable. Compiler determines type - numWithAutoType = {num_with_auto_type}");
        println!("multiple variables declared on one line. even = {even}, odd = {odd}");
    
        // variables are immutable by default. For Safety, concurrency, and speed
        // use the mut keyword to make variables that can be mutated 
        let mut mut_num: i32 = 1;
        println!("mut variables can be changed. mut_num = {mut_num}");
        mut_num = 2;
        println!("mut_num's value has changed = {mut_num}");
    
        // const keyword 
        // must be a const at compile time
        const CONST_USES_UPPERCASE_SNAKE_CASE : f64 = 9.45;
        println!("CONST_USES_UPPERCASE_SNAKE_CASE's value is = {CONST_USES_UPPERCASE_SNAKE_CASE}");
    
        {//this is the start of a block
            let y = 100;
            println!("y is only in scope within the brackets where it was created -- y = {y}");
        }//this is the end of a block
        println!("y is now out of scope");
    
        // shadowing
        let s = 5;
        {//this is the start of a block
            let s = 100;
            println!("while inside the brackets it will use the inner s variable. s = {s}");
        }//this is the end of a block
        println!("once outside the brackets it will use the outter s variable. s = {s}");
    
        fn calc_sum(addend: i32, summand: i32) -> i32 {
            addend + summand
        }
    
        let sum: i32 = calc_sum(2,2);
        println!("sum of 2 plus 2 is {sum}");
    
        // use the name of this project to access code from library file
        // cannot have rust in name
        example::greet();
        
    }

    fn variable_types() {

        
    }


    fn compound_types() {

        // tuples!
        // can hold values of different types
        // tuples arity is 12 with full functionality
        // arity means length, a tuple can only hold 12 values and have full functionality
        // it can hold more but not all functionality works
        let info: (f64, i32, i32) = (5.29, 83, 40);
        //access tuple's data with dot syntax
        let date: f64 = info.0;

        // arrays
        // can hold values but must all be the same types
        // type followed by semicolon and then size(arity) of array
        let buff: [f64; 4] = [1.2, 1.2, 1.2, 1.2];
        // arrays limited to size of 32, after which they lose a lot of functionality
        // access with square brackets
        let num: f64 = buff[0];




    }

    basics()

}
