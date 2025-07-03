use clap::Parser;
use owo_colors::OwoColorize;
use std::{fs, path::{Path, PathBuf}};

#[derive(Debug,Parser)] // 为一个结构体派生 Parser trait 就可以根据结构体的字段自动生成命令行参数解析器
#[command(version,about,long_about = "Better Ls command")] // clap 库中用于自定义命令行参数解析行为的属性（attribute），主要用于为命令行工具添加版本信息的支持
struct CLI{
    path:Option<PathBuf>
}
fn main() {
    let cli = CLI::parse();

    // 解析path,如果没有指定path,则默认使用当前目录的根路径
    let path = cli.path.unwrap_or(
        PathBuf::from(".")
    );

    if let Ok(exist) = fs::exists(&path){
        if exist {
            for file in getFiles(&path){
                println!("{}",file);
            }
        }
        else {
            println!("{}","Path does not exist".red());
        }
    }
    else{
        println!("{}","Error reading directory".red());
    }
    // println!("{}",path.display());
}

fn getFiles(path:&Path) -> Vec<String>{
    let mut data = Vec::default();
    if let Ok(read_dir) = fs::read_dir(path){
        for entry in read_dir{
            if let Ok(file) = entry{
                data.push(
                    file.file_name()
                        .into_string()
                        .unwrap_or("unknown name".into()),
                );
            }
        }
    }
    data
}
