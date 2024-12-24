fn main() {
    let mut a = 10; //可変オブジェクト
    let a_mut_ref = &mut a; //可変参照
    *a_mut_ref = 20;    //参照(仮の所有権)を使用してオブジェクトの操作をするので参照外しを行う。
    println!("{}",a_mut_ref);
}
