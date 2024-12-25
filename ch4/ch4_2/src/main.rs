fn main() {
    let a = 10; //不変オブジェクト
    let a_ref = &a; //参照
    let a_ref_ref = &a_ref; //参照を参照
    // println!("{}",a == a_ref);
    // ↑no implementation for `{integer} == &{integer}`でエラー
    // println!("{}",a_ref_ref == a_ref);
    // ↑ no implementation for `&{integer} == {integer}`でエラー
}
