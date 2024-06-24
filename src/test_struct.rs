use log::{info, warn, error};
#[derive(Debug, Clone)]
pub struct Test {
    pub url: String,
    pub name: String,
    pub weight: i32,
}

impl Test {
    pub fn showMe(&self) -> String {
        format!("{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub struct Color(pub u8, pub u8, pub u8);

#[derive(Debug, Clone)]
pub struct Point(pub f64, pub f64);

pub fn struct_exec(){
        // 结构体测试
        let test = Test {
            url: String::from("http://example.com"),
            name: String::from("测试用的"),
            weight: 1,
        };
        let test1 = Test {
            url: String::from("http://example.com1"),
            ..test.clone()
        };
        // let str = format!("{:#?}", test1);
        info!("{}", test1.showMe());
        // let str = format!("{:#?}", test);
        info!("{}", test.showMe());
    
        let black = Color(0,0,0);
        info!("{}", format!("{:?}", black));
}