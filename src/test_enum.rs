// mod test_struct;
// use test_struct::Test;
use crate::submod::test_submod::exec_submod;
use crate::test_struct::Test;
use log::{debug, error, info, trace, warn};
#[derive(Debug, Clone)]
pub enum TestEnum {
    Papery(Test),
    Electronic(String),
}

pub fn enum_exec() {
    let test_enum = TestEnum::Papery(Test {
        url: String::from("http://enum.test.com"),
        name: String::from("a enum"),
        weight: 10,
    });
    info!("{:?}", test_enum);
    let test_enum = TestEnum::Electronic(String::from("a test"));
    // 获取枚举的内容
    let str = match test_enum {
        TestEnum::Electronic(s) => s,
        _ => String::from("unknown"),
    };

    info!("枚举的内容：{}", str);
    info!("我在test_enum里调用，{:?}", exec_submod())
}
