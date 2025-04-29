/// expr1:expr: This is a matcher that matches an expression.
/// - expr1: This is the name of the matched expression, which can be used in the expander.
/// - :expr: This is the fragment specifier, which indicates the type of Rust construct
///   that we're matching. In this case, expr means "any expression".
///   Here are some common fragment specifiers:
/// - expr:  Matches an expression.
/// - ident: Matches an identifier (e.g., a variable name, function name).
/// - ty:    Matches a type.
/// - stmt:  Matches a statement.
/// - block: Matches a code block (e.g., { ... }).
/// - path:  Matches a path (e.g., module::function, SomeStruct::SomeEnum::SomeVariant).
/// - item: Matches an item (e.g., a function, struct, enum, module).
/// - meta: Matches an attribute, like #[...].
/// - tt:   Matches a single token tree (the most general matcher).
///
/// Macro with one arg.
#[macro_export]
macro_rules! one_arg {
    ($expr1: expr) => {
        println!("Macro with one arg: {}", $expr1);
    };
}

/// Macro with 2 args.
#[macro_export]
macro_rules! two_args {
    ($expr1: expr, $expr2: expr) => {
        println!(
            "Macro with two args: the first {} and the second {}",
            $expr1, $expr2
        );
    };
}

/// Repetition with $()*.
///
/// $($x:expr),*: This is the key to repetition.
/// $x:expr:      Matches an expression, as before.
/// ,:            Matches a comma.
/// $()*:         This is the repetition operator. It means "match zero or more occurrences of the
///               pattern inside the parentheses".
/// ,*:           The comma outside the $() means that the matched expressions should be separated
///               by commas.
#[macro_export]
macro_rules! any_args {
    ($($x:expr),*) => {
        println!("Macro with any args: ");
        $(
            println!("{}", $x);
        )*
    };
}

#[macro_export]
macro_rules! any_args_with_semicolon_separated {
    ($($x:expr);*) => {
        println!("Macro with any args, semicolon separated: ");
        $(
            println!("{}", $x);
        )*
    };
}
