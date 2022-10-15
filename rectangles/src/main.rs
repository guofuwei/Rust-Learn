#[derive(Debug)]
struct Rectangle{
    width:u32,
    height:u32,
}

fn main(){
    let scale=2;
   let rect1=Rectangle{
        width:dbg!(2*scale),
        height:20,
    };
    dbg!(rect1);
}

