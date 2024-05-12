use std::convert::TryInto;

pub mod mandelbrot;
pub mod points;

/**
    Returns the sum 1 + 2 + ... + n
    If n is less than 0, return -1
**/
pub fn gauss(n: i32) -> i32 {
    if n < -1 {
        -1
    }
    else {
        let mut accum = 0;
        for i in 1..=n {
            accum += i;
        }
        accum
    }
}

/**
 * Adds one to the referenced integer.
 */
pub fn add_one(n: &mut i32) {
    *n +=1;
}

/**
 * Modifies the input string so that every character is replaced with 'a'.
 * See https://doc.rust-lang.org/std/string/struct.String.html.
 */
pub fn rewrite_string(s: &mut String) {
    // this technically works but not ideal
    *s = vec!["a"; s.len()].join("");
}

/**
    Converts a binary number to decimal, where each bit is stored in order in the array.
    Index 0 stores the high-order bit.

    Conversion is unsigned (all place values are positive).
    
    Ex: to_decimal of [true, false, true, false] returns 10
**/
pub fn to_decimal(ls: [bool; 32]) -> u32 {
    // let mut decimal:u32 = 0;
    // let mut count:u32 = 32;
    // for i in ls {
    //     count -= 1; 
    //     print!("{count}");
    //     if i == true {
    //         decimal += 2u32.pow(count);
    //     }
    // }
    // decimal

    let mut decimal = 0u32;
    for (idx, bit) in ls.iter().enumerate() {
        if *bit == true {
            decimal += 2u32.pow((31-idx).try_into().unwrap())
        }
    }
    decimal
}

/**
 * The Collatz conjecture (https://en.wikipedia.org/wiki/Collatz_conjecture) is that
 * when one repeatedly applies a function 'f' to a positive integer,
 * the output will eventually be 1. 'f' is defined as follows:
 * 
 * f(n) = n/2 if n is even
 * f(n) = 3n + 1 if n is odd
 * 
 * The function collatz(n) returns the number of times one must apply f to n to get 1.
 * For example, f(4) = 2, f(2) = 1. That is, it took 2 applications of f to get from 4 to 1.
 * Therefore, collatz(4) = 2.
*/
pub fn collatz(mut n: u64) -> u64 {
    let mut res = 0;
    while n != 1 {
        n = if n%2 == 0 { n/2 } else { 3*n +1};
        res +=1;
    }
    res
}


/**
 * Returns a vector containing collatz(1), collatz(2), ..., collatz(n).
 */
pub fn collatz_times(n: u64) -> Vec<u64> {
    let mut vec: Vec<u64> = vec![0;n.try_into().unwrap()];
    for (i, x) in vec.iter_mut().enumerate() {
        let collatz_num: u64 = (i + 1).try_into().unwrap();
        *x = collatz(collatz_num);
    }
    vec
}

/** 
    Takes all of the elements of the given slice and creates a new vector.
    The new vector takes all the elements of the original and rotates them, 
    so the first becomes the last, the second becomes first, and so on.
    
    EX: rotate [1,2,3,4] returns [2,3,4,1]
**/
pub fn rotate(lst: &[i32]) -> Vec<i32> {
    if lst.len() == 0 {
        vec![0;0]
    }
    else if lst.len() == 1 {
        vec![lst[0]]
    }
    else {
        let mut ans = lst[1..].to_vec();
        ans.push(lst[0]);
        ans
    }
}

/**
 * Returns a new string whose contents is the concatenation of the two input slices.
 */
pub fn concatenate_strings(s1: &str, s2: &str) -> String {
    vec![s1,s2].join("") //probs not the most efficient..
}
