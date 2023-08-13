use std::ops::Add;
#[derive(Debug)]
pub struct Personal_info<'a>{
    pub name:&'a str,
    pub age:u8
}
impl<'a> Personal_info<'a>{
    pub fn new()->Personal_info<'a>{
        Personal_info{name:"lzl",age:28}
    }
}
impl <'a>Add for Personal_info{
    fn add(self, rhs: Self) -> Self::Output {
        
    }
}




