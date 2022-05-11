extern crate dfa;

use std::io;
use dfa::DFABuilder;

fn main() {
    // Try to build a DFA for (ab)*
    let builder = DFABuilder::from(&mut io::Cursor::new(
        r#"
        0
        0 a 1
        0 b 2
        1 a 2
        1 b 0
        2 a 2
        2 b 2
        "#,
    ));

    println!("{:#?}", builder);

    let dfa = builder.unwrap().build().unwrap();
    println!("Is valid 'abab'? {}", dfa.is_valid_string("abab"));
    println!("Is valid 'ababa'? {}", dfa.is_valid_string("ababa"));
}
