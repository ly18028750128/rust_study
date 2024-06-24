// 如果在 Windows 7 及以上的 Windows 系统版本中，默认使用的终端命令行是 Powershell，请使用以下命令：

// $env:RUST_BACKTRACE=1 ; cargo run
// 如果你使用的是 Linux 或 macOS 等 UNIX 系统，一般情况下默认使用的是 bash 命令行，请使用以下命令：

// RUST_BACKTRACE=1 cargo run

extern crate greeting as Me;

use chrono::Local;
use env_logger::Builder;
use fern::Dispatch;
use log::info;
use log::LevelFilter;
use std::f64::consts::E;
use std::f64::consts::PI;
use std::io::Write;
use Me::submod::test_submod::exec_submod;
use Me::test_enum::*;
use Me::test_error::catch_error;
use Me::test_lambda::exec_lambda;
use Me::test_struct::*;
use Me::test_t_interface::{exec_test_option_t, Person, Descriptive, Value};

pub fn exec_interface() {
    let value = Person::create(String::from("tom"), 8);
    info!("value.get_age() = {:?}", value);
    info!("value.get_age() = {}", value.get_age());
    info!("value.describe() = {}", value.describe());
}

fn main() {
    // 配置 fern 记录器
    // Initialize the logger with custom formatting
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "[{}] {} on {}:{}  {}",
                record.level(),
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                record.args()
            )
        })
        .filter(None, LevelFilter::Debug)
        .target(env_logger::Target::Stdout)
        .init();

    // 结构体控制测试
    struct_exec();
    // 枚举测试，结合了结构体
    enum_exec();
    // 包结构测试
    exec_submod();
    // 闭包，我觉得就是java的lambda
    info!("{}", exec_lambda(PI, E, PI));
    // 泛化
    exec_test_option_t();
    // 接口
    exec_interface();

    // 捕获错误
    catch_error();

    
}
