fn main() {
    let mut a = 10;
    let a_mut_ref = &mut a; //可変参照
    let a_mut_ref_move = a_mut_ref; //可変参照の移動
    println!("{}",a_mut_ref_move);
}
