fn main() {
    let mut a = 10;
    let a_mut_ref = &mut a; //可変参照
    let a_mut_ref_move = a_mut_ref; //可変参照の移動
    println!("{}",a_mut_ref);
    // これは可変参照は1つしか存在してはいけないためエラーになる(4行目の時点でa_mut_refは存在しない)
}
