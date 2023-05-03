
#[derive(Debug)]
struct Student{
    name:String,
    age:usize,
    score:Score
}


impl Student {
    fn new(name:String,age:usize,chinese_score:usize,math_score:usize,english_score:usize)->Student{
         Student{
            name,
            age,
            score:Score(chinese_score, math_score, english_score)
        }
    }

    fn say(&self){
        println!("My name is {}, my age is {}, my max sore is {}" , self.name , self.age,self.score.max_score())
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

fn main() {
    let student = Student::new(String::from("feitan"), 12,85,80,92);
    student.say();
    println!("{:?}",student)
}
