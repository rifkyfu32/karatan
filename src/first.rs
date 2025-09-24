use crate::third::say_hello as say_hello_third;
pub fn say_hello() {
    println!("Print from first modul");
    say_hello_third();
}
pub(crate) mod second {
    pub(crate) mod third {
        pub fn say_hello() {
            super::super::say_hello();
            super::super::say_hello_third();
        }
    }
}
