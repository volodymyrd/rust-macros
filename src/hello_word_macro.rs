/// #[macro_export] // This attribute makes the macro available for use in other crates.
/// Without it, the macro would only be usable within the current crate.
///
/// macro_rules!: This is the keyword that defines a declarative macro.
/// All declarative macros are defined with macro_rules!.
///
/// (): This is the matcher. It specifies the pattern that the macro invocation must match.
/// In this case, it's an empty set of parentheses, meaning the macro is called with no arguments.
/// Matcher - The pattern that the macro invocation must match.  Matchers are defined within
/// parentheses.
///
/// =>:  This separates the matcher from the expander.
///
/// Expander (or Transcriber): The code that the macro generates when a match is found.
/// Expanders are enclosed in curly braces {}
#[macro_export]
macro_rules! hello_world {
    () => {
        println!("Hello, world!")
    };
}
