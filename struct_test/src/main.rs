
#[derive(Debug)]
struct Student{
    name:String,
    age:usize,
    score:Score,
    sex:Sex
}


impl Student {
    fn new(name:String,age:usize,chinese_score:usize,math_score:usize,english_score:usize,sex:Sex)->Student{
         Student{
            name,
            age,
            score:Score(chinese_score, math_score, english_score),
            sex
        }
    }

    fn say(&self){
        println!("My name is {}, my age is {}, my max sore is {}" , self.name , self.age,self.score.max_score());
        match self.sex {
            Sex::BOY => println!("我是男孩"),
            Sex::GIRL => println!("我是女孩")
        }
    }
}

#[derive(Debug)]
struct Score(usize,usize,usize);


impl Score {
    fn max_score(&self)->usize{
        let num = std::cmp::max(self.0, self.1);
        std::cmp::max(num, self.2)
    }
}
#[derive(Debug)]
enum Sex {
    BOY,GIRL
}

fn main() {
    let student = Student::new(String::from("feitan"), 12,85,80,92,Sex::BOY);
    student.say();
    println!("{:?}",student);

    let student2 = Student::new(String::from("Amy"), 14,90,65,99,Sex::GIRL);
    student2.say();
    println!("{:?}",student2);
}
