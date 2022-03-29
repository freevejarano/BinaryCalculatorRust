use std::io;
use std::cmp::max;

fn input(msg: &str) -> i32 {
    println!("{}", msg);
    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read input");

    match number.trim().parse() {
        Ok(num) => num,
        Err(_e) => panic!("Not a number"),
    }
}

fn to_binary(mut decimal: i32) -> Vec<i32>{
    let mut bin = Vec::new();
    if decimal == 0 {
        bin.push(0);
    } else {
        while decimal > 0 {
            if decimal % 2 == 0 {
                bin.push(0);
            } else {
                bin.push(1);
            }

            decimal /= 2;
        }
    }
    bin
}

fn to_dec(bin : Vec<i32>) -> i32 {
    let mut exp = 0;
    let mut dec: i32 = 0;
    let base: i32 = 2;

    for b in bin {
        if b == 1 {
            dec += base.pow(exp);
        }
        exp += 1;
    }

    dec
}

fn add(mut a : Vec<i32>, mut b: Vec<i32>) -> Vec<i32> {

    if a.len() > b.len() {
        let tmp = a;
        a = b;
        b = tmp;
    }

    let mut carry  = 0;
    let mut i = 0;
    let mut res = Vec::new();

    while i < a.len() {
        let s = a[i] + b[i] + carry;
        if s == 0 {
            res.push(0);
            carry = 0;
        }else if s == 1 {
            res.push(1);
            carry = 0;
        }else if s == 2{
            res.push(0);
            carry = 1;
        }else{
            res.push(1);
            carry = 1;
        }
        i += 1;
    }

    while i < b.len() {
        let s = b[i] + carry;
        if s == 0 {
            res.push(0);
            carry = 0;
        }else if s == 1 {
            res.push(1);
            carry = 0;
        }else if s == 2{
            res.push(0);
            carry = 1;
        }else{
            res.push(1);
            carry = 1;
        }

        i += 1;
    }

    if carry == 1{
        res.push(1);
    }

    res
}

fn sub(mut a : Vec<i32>, mut b: Vec<i32>)  -> Vec<i32> {
    
    let max_len = max(a.len(), b.len());
    if max_len == a.len() {
        while b.len() < max_len {
            b.push(0);
        }
    }else {
        while a.len() < max_len {
            a.push(0);
        }
    }
    
    
    let mut i = 0;
    let mut borrow = 0;
    let mut res = Vec::new();

    while i < max_len {
        let diff = a[i] - b[i] - borrow;
        if diff % 2 == 0 {
            res.push(0);
        }else{
            res.push(1);
        }

        if diff < 0 {
            borrow = 1;
        } else {
            borrow = 0;
        }

        i += 1;
    }

    if borrow  > 0 {
        res.push(1);
    }

    res
}

fn mult(num: i32, num2: i32)->Vec<i32>{
    let mut suma = to_binary(num);
    for _i in 0..num2-1{ //B en decimal
        suma = add(to_binary(num), suma);
    }
    suma
}


fn div(a : Vec<i32>,    b: Vec<i32>) -> Vec<i32> {
    let mut count = 0;
    let mut diff_dec = to_dec(a.clone());
    let mut diff = a;

    while diff_dec > 0 {
        diff = sub(diff.clone(), b.clone());
        diff_dec = to_dec(diff.clone());
        count += 1;
    }

    to_binary(count)
}

fn main() {
    let num1 = input("Enter number 1");
    let num2 = input("Enter number 2");
    let bin1 = to_binary(num1);
    let bin2 = to_binary(num2);
    println!("{:?}", bin1);
    println!("{:?}\n", bin2);
    let sum = add(bin1.clone(), bin2.clone());
    let diff = sub(bin1.clone(), bin2.clone());
    let multi = mult(num1, num2);
    let quo = div(bin1.clone(), bin2.clone());

    let mut sum_reversed = sum.clone();
    sum_reversed.reverse();
    let mut diff_reversed = diff.clone();
    diff_reversed.reverse();
    let mut multi_reversed = multi.clone();
    multi_reversed.reverse();
    let mut quo_reversed = quo.clone();
    quo_reversed.reverse();

    println!("{:?} + {:?} = {:?}\n", bin1, bin2, sum_reversed);
    println!("{} + {} = {}\n", num1, num2, to_dec(sum));
    println!("{:?} - {:?} = {:?}\n", bin1, bin2, diff_reversed);
    println!("{} - {} = {}\n", num1, num2, to_dec(diff));
    println!("{:?} * {:?} = {:?}\n", bin1, bin2, multi_reversed);
    println!("{} * {} = {}\n", num1, num2, to_dec(multi));
    println!("{:?} / {:?} = {:?}\n", bin1, bin2, quo_reversed);
    println!("{} / {} = {}\n", num1, num2, to_dec(quo));
}

