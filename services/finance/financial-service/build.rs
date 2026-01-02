//! 构建脚本
//!
//! 用于编译 Protocol Buffers 定义文件生成 Rust 代码

fn main() {
    tonic_build::configure()
        .compile(
            &["api/proto/finance/v1/journal_entry.proto"],
            &["api/proto"],
        )
        .expect("Failed to compile proto files");

    println!("cargo:rerun-if-changed=api/proto/");
    println!("cargo:rerun-if-changed=build.rs");
}