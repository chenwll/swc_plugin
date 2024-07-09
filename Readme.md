# swc-plugin

rust 编写用于支持底层错误上报，自动捕获 catch 中的错误信息进行上报

# 目录

```
|-- .gitignore
|-- Cargo.lock
|-- Cargo.toml
|-- Readme.md
|-- package.json
|-- .cargo
|   |-- config
|-- src
    |-- lib.rs
    |-- tests
        |-- mod.rs
        |-- uv_test.rs
        |-- common
            |-- mod.rs

```

# 使用方法

xxxxx

# 编写测试及控制行为

## 编写测试

`src/tests` 目录下新建每个模块的测试文件，并统一在 `mod.rs` 文件中引入

```rust
use crate::to;

to!(
    catch_test2,
    // Input codes
    r#"try{
        let a = 1;
    }catch(error){

    }"#,
    // Output codes after transformed with plugin
    r#"
    import {weirwoodErrorReport} from "@common/utils/basicAbility/monitor";
    try{
        let a = 1;
    }catch(error){
        weirwoodErrorReport(error)
    }"#
);
```

## 运行想要的测试用例

1. 运行全部测试用例

   `cargo test`

2. 运行单个测试用例, 只需要指定的测试函数名作为参数即可

   `cargo test catch_test_1`

3. 通过名称来过滤测试, 可以通过指定部分名称的方式来过滤运行相应的测试

   `cargo test catch`

4. 运行某个模块的所有测试用例

   `cargo test uv_test`
