fn main() {
    // Compiler infer type from "Some" because value is present
    let some_number = Some(1);
    let some_string = Some("string");

    // Compiler can't infer type because value is absent
    let absent_number: Option<i32> = None;

    let a: i8 = 5;
    let b: Option<i8> = Some(5);

    //let c = a + b; // Compile error
                    // Compiler ensures we have "value" before using it elsewhere
                    // Need to convert Option<T> to <T> before using <T> elsewhere

    // When we use something that isn't Option<T>, we can be confident value isn't null
    // When we use Option<T>, compiler forces us to handle case of value being null
}