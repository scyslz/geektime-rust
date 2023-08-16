fn main() {
    let data: String = "hello".into();

    let s1: &str = &data;
    let s2: &str = &data;
    let s3: &str = &data;

    // 打印栈地址
    dbg!(&s1 as *const _);
    dbg!(&s2 as *const _);
    dbg!(&s3 as *const _);
    println!("{:p}",&s1);
    println!("{:p}",&s2);
    println!("{:p}",&s3);

    // 打印堆地址
    dbg!(s1.as_bytes() as *const _);
    dbg!(s2.as_bytes() as *const _);
    dbg!(s3.as_bytes() as *const _);
    println!("{:p}",s1);
    println!("{:p}",s2);
    println!("{:p}",s3);
}
