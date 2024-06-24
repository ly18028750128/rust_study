// mod test_struct;
// use test_struct::Test;
use crate::test_struct::Test;
use crate::submod::test_submod::exec_submod;
use log::{info, warn, error, debug, trace};
#[derive(Debug, Clone)]
pub enum TestEnum {
    Papery(Test),
    Electronic(String),
}



pub fn enum_exec() {
    let test_enum = TestEnum::Papery(Test{
        url: String::from("http://enum.test.com") ,
        name: String::from("a enum"),
        weight: 10
    });
    info!("{:#?}", test_enum);
    let test_enum = TestEnum::Electronic(String::from("a test"));
    info!("{:?}", test_enum);
    exec_submod()
}