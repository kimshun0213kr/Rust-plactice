fn main() {
    let mut a = 10;
    let a_ref1 = &a;    //不変参照
    let a_mut_ref1 = &mut a;    //可変参照
    let a_mut_ref2 = &mut a;    //可変参照. この時点でa_ref1, a_mut_ref1は存在しない
    let a_ref2 = &a;    //不変参照. この時点でa_mut_ref2は存在しない。
    // println!("{},{},{}",a_ref1,a_mut_ref1,a_mut_ref2)
    // ↑はエラーになる。
    println!("{}",a_ref2)
}
