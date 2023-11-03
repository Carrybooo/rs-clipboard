//#![allow(non_snake_case)]
#![allow(unused)]

use std::fs::{self, File};
use std::io;
use std::path::{Path, PathBuf};
use std::env;
use std::process::exit;
use structopt::StructOpt;

fn main() -> io::Result<()> {

    let args_array: Vec<String> = env::args().collect();

    let args_parsed = Opt::from_args();
    println!("{:?}",&args_parsed);

    let (command, clipboard_id) = parse_command(args_parsed.commmand);
    let args = &args_parsed.args;
    println!("command :{}, id :{:?}, args: {:?}", command, clipboard_id, args);



    let mut clipboard_name = String::from("0");
    if clipboard_id != None {
        clipboard_name = clipboard_id.unwrap();
    }
    println!("cb_name : {}", clipboard_name);

    exec_command(command.as_str(), &clipboard_name.as_str(), args);





    // --------------------------------------------------------------------

    // let read_result = read_raw_cb(&clipboard_path)?;
    // println!("read result :\n{}", read_result);

    // let write_result = write_raw_cb(&clipboard_path, "Hello, world !".to_string())?;
    // println!("writing data… {:?}", write_result);

    // let read_result = read_raw_cb(&clipboard_path)?;
    // println!("read result :\n{}", read_result);
    // --------------------------------------------------------------------

    Ok(())
}


fn exec_command(command: &str, clipboard_name: &str, args: &Vec<String>) -> io::Result<()>{
    let clipboards_dir = create_clipboards_dir()?;
    let clipboard_path = clipboards_dir.join(clipboard_name);

    match command {
        "copy" => {
            if std::path::PathBuf::from(args.concat()).is_file(){
                fs::copy(args.concat(), &clipboard_path).map(|_| ()) //Result<u64> to Result<()>
            }else{
                let file_name = args[0].clone();
                write_raw_cb(&clipboard_path, file_name)
            }
        },
        "paste" => {
            //println!("cb_path : {:?}", &clipboard_path);
            let data = read_raw_cb(&clipboard_path).unwrap();
            if &args.len() > &0 {
                let path = &args[0];
                write_raw_cb(&std::path::PathBuf::from(path), data)
            }else{
                println!("{}",data); Ok(())
            }
        },
        _ => Err(io::Error::new(io::ErrorKind::Other, "Error during command execution."))
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "clipboard")]
struct Opt {
    ///Command
    commmand: String,

    ///Optionnal arguments
    args: Vec<String>,
}

fn parse_command(cmd_block: String) -> (String, Option<String>){
    if cmd_block.starts_with("cp") || cmd_block.starts_with("copy"){
        match cmd_block.trim_start_matches("cp").trim_start_matches("copy") {
            "" => (String::from("copy"), None),
            rest => (String::from("copy"), Some(String::from(rest))),
        }
    }else if cmd_block.starts_with("ps") || cmd_block.starts_with("paste"){
        match cmd_block.trim_start_matches("ps").trim_start_matches("paste") {
            "" => (String::from("paste"), None),
            rest => (String::from("paste"), Some(String::from(rest))),
        }
    }else if cmd_block.starts_with("ct") || cmd_block.starts_with("cut"){
        match cmd_block.trim_start_matches("ct").trim_start_matches("cut") {
            "" => (String::from("cut"), None),
            rest => (String::from("cut"), Some(String::from(rest))),
        }
    }
    else {(String::from("unknown"), None)}
}

fn create_clipboards_dir() -> io::Result<PathBuf> {
    let home_dir = home::home_dir().ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Failed to get home directory"))?;
    let clipboards_dir = home_dir.join(".local/state/clipboard");
    fs::create_dir_all(&clipboards_dir)?;
    Ok(clipboards_dir)
}

fn read_raw_cb(path: &PathBuf) -> io::Result<String> {
    fs::read_to_string(path)
}

fn write_raw_cb(path: &PathBuf, content: String) -> io::Result<()>{
    if !std::path::Path::is_file(path) {fs::File::create(path)?;}
    fs::write(path, content)
}

fn print_help(arg_count: usize){
    println!("Incorrect argument count : {}", arg_count)
}