use std::fs::File;
use std::io::{self, ErrorKind, Read};
fn main() {
    let f= match File::open("hello.txt"){
        Err(e)=> match e.kind(){
            ErrorKind::NotFound=> match File::create("hello.txt"){
                Err(e)=>panic!("Error while creating the file {e}"),
                Ok(file)=>file
            },
            other=>panic!("Error while opening the file {other}"),
        },
        Ok(file)=>file
    };

}
fn read_username_from_file()->Result<String,io::Error>{
    let username_file_res=File::open("hello.txt");

    let mut username_file=match username_file_res{
        Err(e)=>return Err(e),
        Ok(f)=>f
    };

    let mut username=String::new();
    match username_file.read_to_string(&mut username){
        Err(e)=>Err(e),
        Ok(_)=>Ok(username)
    }
}

fn read_username_from_file_qmark_operator()->Result<String,io::Error>{
    let mut username_file=File::open("hello.txt")?;

    let mut username=String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
} 
