fn main() {
    {  // s is not valid here = Error
        let s = "Hello";  // s is valid here = Valid
        // s is still here = Valid
    }  // scope is over = Error

    let mut s = String::from("hello");  // Not a string literial, but a String. Heap.

    s.push_str(", world!");  // push_str() appends a literal to a string
    println!("{s}");  // This will print `hello, world`

    /// Q: How does Rust handle deallocating heaped memory, such as Strings?
    /// A: An object will have a `drop` function which is called when the 
    ///    object goes out of scope. Once outside the curly brackets, `drop`
    ///    is called and the memory is freed. No need for a GC!
    
    // Set x to y, and set x to a copy of y. Simple. Equal in size
    let x = 5;
    let y = x;

    // String data is copied (pointer, length, and capacity). Do not copy the heap data
    let s1 = String::from("hello");
    let s2 = s1;

    /// To ensure memory safety, s1 is no longer valid and if used, will
    /// throw an Invalidated Reference error. s1 was `moved` to s2. Since
    /// this happens at compile time, it ensures memory safety.
    
    let mut s = String::from("hello");
    s = String::from("ahoy");  // Nothing points to `hello` anymore, out of scope, and therfore freed

    let s1 = String::from("hello");
    let s2 = s1.clone();  // Deep-copy, this works!

    println!("s1 = {s1}, s2 = {s2}");

    /// The below code (which we saw before) seems not contradict what we just learnt.
    /// Because `5` has a known size at compile time, and therfore on the stack, it is
    /// not shallow-copied, but is deep-copied and works.
    let x = 5;
    let y = x;
}
