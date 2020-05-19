// Move this to unit test and more heavier example here later.

use publish::{
    nested_macro,
    public_struct,
};

public_struct!(
    pub struct MessageBase {
        pub text: String
    }
);

MessageBase!(); // You have to call it to use.

// $cargo test -- --nocapture
#[test]
fn public_struct() {
     // You have to call it to use.
    let message = MessageBase {
        text: "First Message".into(),
    };
    println!("{}", &message.text);
}
