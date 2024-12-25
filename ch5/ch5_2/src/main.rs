fn print_coodinates(&(x,y): &(i32,i32)) {
    println!("location by func1: ({},{})",x,y);
}

fn main() {
    let point = (3,5);
    print_coodinates(&point);
}
