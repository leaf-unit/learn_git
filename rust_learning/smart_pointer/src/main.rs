//mod box1;
//mod arc1;
mod Mytype;
fn main() {
    let a = Mytype::Personal_info::new();
    let b = Mytype::Personal_info{
        name:"leaf",
        age:99
    };
    let c =a+b;
    println!("{:?}",b);
}
