use std::env;
use minigrep::CmdParam;
fn main() {
    let param = parse_cmd().unwrap_or_else(|error|{
        println!("{}",error);
        std::process::exit(1);
    });
    param.search().unwrap_or_else(|error|{
        println!("{}",error);
        std::process::exit(1);
    });
}


fn parse_cmd()->Result<CmdParam,String>{
    let args:Vec<String> = env::args().collect();
    CmdParam::new(args)
}