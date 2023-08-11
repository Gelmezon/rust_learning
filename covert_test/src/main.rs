
fn main() {

    println!("num of logical cores: {}", num_cpus::get());

    let a: i32 = 10;
    let b: u32 = 20;

    if a > b as i32 {
        println!("a > b");
    } else {
        println!("a <= b");
    }

    let mut values: [i32; 2] = [1, 2];
    let p1: *mut i32 = values.as_mut_ptr();
    let first_address = p1 as usize;
    let second_address = first_address + 4;
    let p2 = second_address as *mut i32;
    unsafe {
        *p2 += 3;
    }
    println!("values[1] = {}", values[1]);

    let a: u8 = 10;
    let b: u16 = 1200;

    let b_: u8 = match b.try_into() {
        Ok(b1) => b1,
        Err(e) => {
            println!("{:?}", e.to_string());
            0
        }
    };
    if a <= b_ {
        print!("a <= b_");
    } else {
        println!("a > b_")
    }

    let foo = Foo{
        x:10,
        y:10
    };
    let bar = reinterpret(foo);
    println!("{:?}",bar);

    let pointer = example_func as *const();
    let function = unsafe {
        std::mem::transmute::<*const(),fn()->i32>(pointer)
    };
    println!("{}",function())
}

struct Foo {
    x: u32,
    y: u16,
}

#[derive(Debug)]
struct Bar {
    a: u32,
    b: u16,
}

fn reinterpret(foo: Foo) -> Bar {
    let (x,y)= (foo.x,foo.y);
    Bar{
        a:x,
        b:y
    }
}

fn example_func()-> i32{
    0
}


