// run-rustfix
#![warn(clippy::crate_in_macro_def)]

#[macro_use]
mod hygienic {
    macro_rules! print_message_hygienic {
        () => {
            println!("{}", $crate::hygienic::MESSAGE);
        };
    }

    pub const MESSAGE: &str = "Hello!";
}

#[macro_use]
mod unhygienic {
    macro_rules! print_message_unhygienic {
        () => {
            println!("{}", crate::unhygienic::MESSAGE);
        };
    }

    pub const MESSAGE: &str = "Hello!";
}

fn main() {
    print_message_hygienic!();
    print_message_unhygienic!();
}
