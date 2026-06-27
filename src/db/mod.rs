#[derive(Debug)]
pub struct MyStruct {
    pub public_var: i64,
    private_var: i64,
}

impl MyStruct {
    pub fn new(public_var: i64) -> Self {
        MyStruct {
            public_var,
            private_var: 32,
        }
    }
}
