
fn main() {
    let v: Vec<i32> = vec![1, 2, 3];

}


#[macro_export]
macro_rules! calc {
    ( $( $x:expr ), *) => {
    };
}