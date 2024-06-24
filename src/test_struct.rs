use log::info;
#[derive(Debug, Clone)]
pub struct Test {
    pub url: String,
    pub name: String,
    pub weight: i32,
}

impl Test {
    pub fn show_me(&self) -> String {
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
        // 这里要用clone,通过drive宏注入，要不然打印test会报所有被移走的异常
        let test1 = Test {
            url: String::from("http://example.com1"),
            ..test.clone()
        };
        
        // let str = format!("{:#?}", test1);
        info!("{}", test1.show_me());
        // let str = format!("{:#?}", test);
        info!("{}", test.show_me());
    
        let black = Color(0,0,0);
        info!("{}", format!("{:?}", black));
}