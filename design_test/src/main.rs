
trait Action{
    fn act(&self);
}

struct AAction{
    name:String,
}
struct BAction{
    name:String,
}

impl Action for AAction{
    fn act(&self){
        println!("AAction,{}",&self.name);
    }
}

impl Action for BAction{
    fn act(&self){
        println!("BAction:{}",&self.name);
    }
}

struct User<T> where T:Action{
    action:T,
}

impl<T:Action> User<T>{
    fn act(&self){
        self.action.act();
    }
}
    

fn main(){
    let a = AAction{
        name:"amy".to_string(),
    };
    let b = BAction{
        name:"bob".to_string(),
    };
    let u = User{
        action:a,
    };
    u.act();
    let u = User{
        action:b,
    };
    u.act();
}