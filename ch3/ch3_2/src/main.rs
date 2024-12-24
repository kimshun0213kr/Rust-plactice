fn main() {
    let a = 10; //可変オブジェクト
    let a_ref = &a; //参照. この場合でもTraitは維持される。
    let a_ref_copy = a_ref; //コピー
    println!("{},{},{}",a,a_ref,a_ref_copy);
}
