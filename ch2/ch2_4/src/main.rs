fn main() {
    let mut a = 10;
    let a_ref1 = &a;    //不変参照
    let a_mut_ref1 = &mut a;    //可変参照
    let a_mut_ref2 = &mut a;    //可変参照
    println!("{}",a_ref1);
    // 可変参照と不変参照が同時に存在してしまうためエラーになる
}
