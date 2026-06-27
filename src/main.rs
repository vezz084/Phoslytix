use crate::db::MyStruct;

mod db;

fn main() {
    println!("Hello, world!");
    let mystr: MyStruct = MyStruct::new(62);

    dbg!(&mystr);
}
