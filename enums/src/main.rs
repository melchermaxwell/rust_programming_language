impl Message {
    fn call(&self) {
        //method body
    }
}

fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();
}
