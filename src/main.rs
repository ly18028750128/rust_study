extern crate greeting as Me;

use Me::test_struct::*;
use Me::test_enum::*;
use Me::submod::test_submod::exec_submod;
use fern::Dispatch;
use chrono::Local;


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
    exec_submod()
}

