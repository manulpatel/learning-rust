fn main() {
    println!("Hello World without Cargo!");
}

//NOTES

//Variables
//let,const,static (Use mut to make mutable)
//Rust is statically typed;So need to explicilty decalre type of varible except using 'let'.

//Naming Convention
//{Variable,Function,Files}=snake_case
//{Constants, Statics}=SCREAMING_SNAKE_CASE
//{Type,Traits,Enums}=PascalCase

//Functions
//fn main(){                            //Here the return type is empty tuple -> ()
//    let _unused_variable=my_func(10);
//}            
//fn my_func(x:u8) -> i32 {
//    x as i32
//}
//

//Strings and Slices
//let my_str: &str = "my_str is a ref to string literal, stack stored, its value can't change and its size is fixed."
//let my_string: String = String::from("my_string is an instance of the String struct, heap allocated, can have unknown size at comiple time");

//String literal == string slice as &str refers to the part of the string.
//let my_string = String::from("The quick brown fox");
//let my_str: &str = &my_string[4..9]; // "quick"

//let my_arr: [usize; 5] = [1, 2, 3, 4, 5];
//let my_arr_slice: &[usize] = &my_arr[0..3]; // [1, 2, 3]
//The [T; n] notation is used to create an array of n elements of type T.

