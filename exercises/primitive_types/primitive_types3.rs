// primitive_types3.rs
// Create an array with at least 100 elements in it where the ??? is.
// Execute `rustlings hint primitive_types3` for hints!

fn main() {
    let a = create_array();

    fn create_array() -> Vec<i8> {
        let mut arr = Vec::new();
        for i in (1..105) {
            arr.push(i);
        }
        arr
    }

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
