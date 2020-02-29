const MAXVAL : f32 =5000.0;
fn main() {
    println!("Hello, world!");
    println!("Hello, world!");

    let x:f32 = -5.3;
    println!("The value of x is {} ", x);
 
    let x=x+5.2;
    println!("The value of x is {} ", x);

    let x =2*x as i32;
    println!("The value of x is {} ", x);
 //   let MAXVAL = MAXVAL*2.2;
    println!("The value of x is {} ", MAXVAL);
    let ch = "Here we go ";
    let l = ch.len();
    println!("Ch length is {}",l);

    let ar = [10,20,49];
    println!("Array is {:?}",ar);
    println!("Array is {:?}",ar[2]);

    let tu = ("Ali","Ahmed" ,30,40);
    
    println!("Tuple is {:?}",tu);
    let c = 10;
    if c < 0
    {
        println!("c is less than zero");
    }
    else{
        println!("c is greater than zero");

    }
    let mut v = 0;
    loop
    {
        println!("Here we go {} ", v);
        v=v+1;
        if v == 5
        {
            break;
        }
       
    }
    println!("For loop ");
    for l in (1..4). rev()
    {
        println!("For loop is {}",l);

    }
    let arr:[i32;4] = [1,2,3,4];
    for l in arr.iter()
    {
        println!("value are {} ",l);
    }
    let x = 5;
let y = {
let x = 3;
x + 1
};
println!("The value of y is: {}", y);

    let mut s = String::from("here we go ");
    s.push_str(", bro");
    println!("{}",s);

    let s = fun(x,s);
    println!("{:?}",s);

    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
   
    println!("{} {}", r1, r2); 
    let r3 = &mut s; // no problem
  
    println!("{}", r3);


let mut x = 5; // x comes into scope


//makes_copy(x);
x=x+2;
let e=x;
println!("{}",x);
println!("{}",e);

let s = String::from("hello");
let len = s.len();
let slice = &s[3..len];
println!("slice value is = {}", slice);

let slice = &s[..];
println!("slice value is = {}", slice);

let bytes = s.as_bytes();
for (i, &item) in bytes.iter().enumerate() {
if item == b' ' {
return &s[0..i];
}
}

}

fn fun(s :i32, x: String)->(i32,String)
{
    println!("{}",x);
    (s,x)
}