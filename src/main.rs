extern crate greeting as Me;

use log::{info};
use chrono::Local;
use fern::Dispatch;
use Me::submod::test_submod::exec_submod;
use Me::test_enum::*;
use Me::test_lambda::exec_lambda;
use Me::test_struct::*;

fn main() {
    // 配置 fern 记录器
    Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} [{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .apply()
        .unwrap();

    // 结构体控制测试
    struct_exec();
    // 枚举测试，结合了结构体
    enum_exec();
    // 包结构测试
    exec_submod();

    info!("{}",exec_lambda(1, 2, 3));
}
