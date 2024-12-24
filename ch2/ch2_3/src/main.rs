fn main() {
    let mut a = 10;  //可変オブジェクト
    let a_ref1 = &a; //参照
    let a_mut_ref1 = &mut a; //可変参照
    let a_mut_ref2 = &mut a; //この時点でa_ref1,a_mut_ref1は存在しないことになる
    *a_mut_ref1 = 20;
    println!("{}",a);
    // 可変参照が複数存在してしまうためエラーになる
}
