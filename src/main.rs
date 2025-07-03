use chrono::{DateTime, Utc};
use clap::Parser;
use owo_colors::OwoColorize;
use serde::Serialize;
use strum::Display;
use tabled::{settings::{object::{Columns, Rows}, Color, Style}, Table, Tabled};
use std::{fs, path::{PathBuf}};

#[derive(Debug,Display,Serialize)]
enum EntryType {
    File,
    Dir,  
}

// store meta data
#[derive(Debug, Tabled,Serialize)]
struct FileEntry{
    #[tabled{rename="Name"}]
    name:String,
    #[tabled{rename="Type"}]
    e_type:EntryType, 
    #[tabled{rename="Size B"}]
    len_bytes:u64,
    #[tabled{rename="Modified"}]
    modified:String,
}

#[derive(Debug,Parser)] // 为一个结构体派生 Parser trait 就可以根据结构体的字段自动生成命令行参数解析器
#[command(version,about,long_about = "Better Ls command")] // clap 库中用于自定义命令行参数解析行为的属性（attribute），主要用于为命令行工具添加版本信息的支持
struct CLI{
    path:Option<PathBuf>,
    #[arg(short,long)]
    json: bool,
}

fn main() {
    let cli = CLI::parse();

    // 解析path,如果没有指定path,则默认使用当前目录的根路径
    let path = cli.path.unwrap_or(
        PathBuf::from(".")
    );

    if let Ok(exist) = fs::exists(&path){
        if exist {
            if cli.json{
                let files = get_files(&path);
                println!(
                    "{}",
                    serde_json::to_string(&files).unwrap_or("cannot parse json".to_string())
                );
            }
            else{
                print_table(path);
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

fn get_files(path:&PathBuf) -> Vec<FileEntry>{
    let mut data = Vec::default();
    if let Ok(read_dir) = fs::read_dir(path){
        for entry in read_dir{
            if let Ok(file) = entry{
                map_data(file, &mut data);
           }
        }
    }
    data
}

fn map_data(file: fs::DirEntry,data: &mut Vec<FileEntry>){
    if let Ok(meta) = fs::metadata(&file.path()){
        data.push(
            FileEntry { 
                name: file.file_name()
                    .into_string()
                    .unwrap_or("unknown name".into()),
                len_bytes: meta.len(),
                e_type: if meta.is_dir(){
                    EntryType::Dir
                }else{
                    EntryType::File
                },
                modified: if let Ok(modified) = meta.modified(){
                    let date:DateTime<Utc> = modified.into();
                    format!("{}",date.format("%a %b %e %Y"))
                }else{
                    String::default()
                },
        }
    );
    }
}

fn print_table(path:PathBuf){
            let files = get_files(&path);
            let mut table = Table::new(files);
            table.with(Style::rounded());
            table.modify(Columns::first(), Color::FG_BRIGHT_CYAN);
            table.modify(Columns::one(2), Color::FG_BRIGHT_MAGENTA);
            table.modify(Columns::one(3), Color::FG_BRIGHT_YELLOW);
            table.modify(Rows::first(), Color::FG_BRIGHT_GREEN);
            println!("{}",table);
 
}