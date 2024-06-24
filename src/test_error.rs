use log::info;
use std::io;
#[derive(Debug)]
pub enum MyError {
    IoError(io::Error),
    TestError(String),
}

fn panic_error(i_error: i8) -> Result<String, MyError> {
    if i_error < 0 {
        return Err(MyError::TestError(String::from("失败调用")));
    }
    Ok(String::from("成功调用"))
}

pub fn catch_error() {
    // 成功调用
    let f = panic_error(1);
    match f {
        Ok(file) => {
            info!("{}", file);
        }
        Err(err) => match err {
            MyError::IoError(_) => todo!(),
            MyError::TestError(_) => {
                info!("{:?}", err);
            }
        },
    }
    // 失败调用
    let f = panic_error(-1);
    match f {
        Ok(file) => {
            info!("{}", file);
        }
        Err(err) => match err {
            MyError::IoError(_) => todo!(),
            MyError::TestError(msg) => {
               
                info!("{}", msg);
            }
        },
    }
}
