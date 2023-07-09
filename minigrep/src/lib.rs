use std::fs::File;
use std::fs;
pub struct CmdParam{
    pub query: String,
    pub file_path: String,
}

impl CmdParam{
    pub fn new(args:Vec<String>)->Result<CmdParam,String>{
        let mut args_iterr = args.into_iter();
        args_iterr.next();
        let query = match  args_iterr.next(){
            Some(args) => args,
            None => return Err("缺少参数".to_string()),
        }; 
        let file_path = match  args_iterr.next(){
            Some(args) => args,
            None => return Err("缺少参数".to_string()),
        }; 
        return Ok(CmdParam{
            query,
            file_path,
        });

    }
    pub fn search(&self)->Result<(),String>{
        let res = File::open(self.file_path.as_str());
        if res.is_err() {
            return Err(format!("打开文件失败：{}",self.file_path).to_string());
        }
        let contents = fs::read_to_string(self.file_path.as_str()).unwrap();
        let info_array = handle_search(&self.query, contents.as_str());
        for info in info_array{
            println!("{}",info);
        }
        Ok(())
    }
}


pub fn handle_search<'a>(query:&str,contents:&'a  str)->Vec<&'a str>{
    contents.lines().filter(|line|{
        line.contains(query)
    }).collect()
}