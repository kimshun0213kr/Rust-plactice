fn main() {
    let mut a = 10; //可変オブジェクト(可変参照)
    let a_ref1 = &a;
    let a_mut_ref1 = &mut a;    //可変参照
    let a_mut_ref2 = &mut a;    //可変参照
    *a_mut_ref1 = 20;
    println!("{}",a);
    // 可変参照が複数あるけれども、借用チェック時に制約を満たしているためエラーにならない
}
