// 如果在 Windows 7 及以上的 Windows 系统版本中，默认使用的终端命令行是 Powershell，请使用以下命令：

// $env:RUST_BACKTRACE=1 ; cargo run
// 如果你使用的是 Linux 或 macOS 等 UNIX 系统，一般情况下默认使用的是 bash 命令行，请使用以下命令：

// RUST_BACKTRACE=1 cargo run

extern crate greeting as Me;

use log::{info};
use chrono::Local;
use fern::Dispatch;
use Me::submod::test_submod::exec_submod;
use Me::test_enum::*;
use Me::test_lambda::exec_lambda;
use Me::test_struct::*;
use Me::test_error::catch_error;
use std::f64::consts::PI;
use std::f64::consts::E;


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
    // 闭包，我觉得就是java的lambda
    info!("{}",exec_lambda(PI, E, PI));
    catch_error()
}
