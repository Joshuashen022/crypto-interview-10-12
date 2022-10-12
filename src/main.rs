mod base_algorithm;
mod pbkdf2_cipher;
mod point;

use base_algorithm::Plan;
use k256::{
    elliptic_curve::{generic_array::arr, group::ff::PrimeField, ops::LinearCombination},
    ProjectivePoint, Scalar,
};
use num::{BigInt, BigUint};
use point::Point;

fn main() {
    let p = BigUint::parse_bytes(
        b"115792089237316195423570985008687907853269984665640564039457584007908834671663",
        10,
    );
    let a = BigUint::parse_bytes(
        b"37495995483093812530829120344068921073950778374277050857635845226183990889532",
        10,
    );
    let b = BigUint::parse_bytes(
        b"36273884976317350876892933450181613438664462160902682135941368945682163872771",
        10,
    );
    let res = a.unwrap().modpow(&b.unwrap(), &p.unwrap());
    println!("{}", res);

    let p1_x = BigInt::parse_bytes(
        b"36034668029310999675425029227919426304128362788024891102120850317866231552679",
        10,
    );
    let p1_y = BigInt::parse_bytes(
        b"81120990977494636963407451456021843404486499021598452981689548730055179196713",
        10,
    );

    let p2_x = BigInt::parse_bytes(
        b"17178020516540951919986460933710490672232047574774824837208169858689311129064",
        10,
    );
    let p2_y = BigInt::parse_bytes(
        b"71957217096292920627957410906773462576199313707110833846387209016083557649656",
        10,
    );

    let p1 = Point::new(p1_x.unwrap(), p1_y.unwrap());
    let p2 = Point::new(p2_x.unwrap(), p2_y.unwrap());

    println!("- p1 = {:?}", p1.neg());
    // println!("p2 {:?}", p2);

    let p_new = p1.clone() + p2;
    println!("p1 + p2 = {:?}", p_new);

    let k = BigInt::parse_bytes(
        b"112722522736802425171074620119739342837016662713926899217486478633056306669418",
        10,
    );
    let mul_result = p1.mul(k.unwrap());
    println!("p1 * k = {:?}", mul_result);

    pbkdf2_cipher::test();
    let array = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let k = 10;
    let res = base_algorithm::algorithm_test1(array, k);
    println!("Plans are {:?}", res);
    let mut sum = 0;
    for p in res {
        sum += p.sum();
    }
    println!("Minimum sum is {:?}", sum);

    let sss = String::from("|**|**|*");
    let starts = vec![-1];
    let ends = vec![2, 3];
    let sum = base_algorithm::algorithm_test2(sss, starts, ends);
    for c in sum {
        println!("{:?}", c)
    }
}
