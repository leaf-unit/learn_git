
fn fbn(i:u32)->u32{
    if i==0
    {
        0
    }
    else {
        fbn(i-1)+fbn(i-2)
    }
}
fn main() {
    fbn(10);
}
