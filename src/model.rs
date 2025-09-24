pub struct User {
    pub id: String,
    pub name: String,
    pub age: u8,
}
impl User {
    pub fn say_hello(self, name: &str) {
        println!("Hello {}, my name is {}", name, self.name);
    }
}
