fn main() {
    let _s1 = String::from("hello"); // s1 is the owner of the String
    let _s2 = _s1; // Ownership of the String is moved to s2
     println!("{}", _s1); // Error: s1 is no longer the owner
    //println!("{}", s2); // s2 can use the String
}



fn main() {
    {//not valid
        let s = String::from("hello"); // s is valid from this point
        println!("{}", s);
    } // s goes out of scope and the String is dropped (memory is freed)
}

fn main() {
    let book = String::from("The Rust Programming Language"); // Owner of the book
    let reader1 = &book; // Immutable borrow by reader1
    let reader2 = &book; // Immutable borrow by reader2

    // Both reader1 and reader2 can read the book
    println!("Reader 1 is reading: {}", reader1);
    println!("Reader 2 is reading: {}", reader2);

    // book is still available for reading because ownership hasn't been transferred
}

fn main() {
    let mut book = String::from("The Rust Programming Language"); // Owner of the book
    let editor = &mut book; // Mutable borrow by editor

    editor.push_str(" - Annotated Edition"); // Modify the book

    // book is modified, and no other borrows are allowed during the modification
    println!("The edited book is: {}", editor);

    // After editor goes out of scope, the book can be borrowed again
}


fn main() {
    let book = String::from("The Rust Programming Language"); // Owner of the book

    {
        let reader = &book; // Immutable borrow within a smaller scope
        println!("Reader is reading: {}", reader);
    } // reader goes out of scope here, and the borrow ends

    println!("Book is available for borrowing again: {}", book);
}


