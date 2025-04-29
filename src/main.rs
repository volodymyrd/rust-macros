mod hello_word_macro;
mod macros_with_arguments;

fn main() {
    hello_world!();

    one_arg!("Hello");
    one_arg!(1 + 2);

    two_args!("Hello", "World");
    two_args!(5 * 2, 4 / 3);

    any_args!(1, "hello", 1.14, true);
    any_args!(10, 20, 30);

    // Can handle empty case
    any_args!();

    any_args_with_semicolon_separated!(10; 20; 30);
}
