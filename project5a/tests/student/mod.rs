
extern crate part0;

use part0::*;
// use part0::mandelbrot::*;
// use part0::points::*;


#[test]
fn public_test_decimal_shermy() {
    let xs = [
        false, false, false, false, false, false, false, false, 
        false, false, false, false, false, false, false, false, 
        false, false, false, false, false, false, false, false, 
        false, false, false, false, false, false, false, true];
    assert_eq!(1, to_decimal(xs));

    let xs = [
        true, false, false, false, false, false, false, false, 
        false, false, false, false, false, false, false, false, 
        true, false, false, false, false, false, false, false, 
        false, false, false, false, false, false, false, true];
    assert_eq!(2147516417, to_decimal(xs));


}