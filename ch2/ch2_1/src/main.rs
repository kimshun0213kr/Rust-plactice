fn main() {
    let b: i32 = 0;
    let d = 10;  //不変なオブジェクト
    let aref1 = &d ;    //参照
    let aref2 = &d;     //参照
    println!("{},{},{}",d,aref1,aref2); //チェック
}
