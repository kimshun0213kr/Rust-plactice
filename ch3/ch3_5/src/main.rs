fn copy_trait_check<T: Copy>(_: T) {}

fn main() {
    // let s = String::from("hello");
    // copy_trait_check(s);
    // the trait `Copy` is not implemented for `String`と、↑はエラーになる。
    let a = 10;
    copy_trait_check(a)
}