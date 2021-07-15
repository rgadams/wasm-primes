mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn sieve_of_eratosthenes(n: usize) -> Vec<usize> {
    crate::utils::set_panic_hook();

    let mut prime_array = vec![1; n];
    prime_array[0] = 0;
    prime_array[1] = 0;
    let root_n = js_sys::Math::sqrt(n as f64) as usize;
    for i in 2..root_n+1 {
        if prime_array[i] == 1 {
            let mut j = i.pow(2);
            while j < n {
                prime_array[j] = 0;
                j = j + i;
            }
        }
    }
    filter_prime_list(prime_array)
}

fn filter_prime_list(prime_array: Vec<usize>) -> Vec<usize> {
    let mut filtered_prime_array = Vec::new();
    for i in 0..prime_array.len() {
        if prime_array[i] == 1 {
            filtered_prime_array.push(i);
        }
    }
    filtered_prime_array
}

// algorithm Sieve of Eratosthenes is
//     input: an integer n > 1.
//     output: all prime numbers from 2 through n.

//     let A be an array of Boolean values, indexed by integers 2 to n,
//     initially all set to true.
    
//     for i = 2, 3, 4, ..., not exceeding √n do
//         if A[i] is true
//             for j = i2, i2+i, i2+2i, i2+3i, ..., not exceeding n do
//                 A[j] := false

//     return all i such that A[i] is true.