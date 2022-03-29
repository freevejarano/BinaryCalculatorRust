use std::io;
use std::cmp;


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

fn sum(mut a: Vec<i32>, mut b: Vec<i32>)->Vec<i32>{
    let max_len = cmp::max(a.len(), b.len());

    while a.len() != max_len || b.len()!=max_len{
        if a.len() != max_len {
            a.push(0);
        }else if b.len() != max_len {
            b.push(0);
        }
    }

    let mut result = Vec::new();
    let mut carry = 0;
    let mut i = 0;

    for i in 0..max_len{
        let mut r = carry;
        r = r + a[i];
        r = r + b[i];
        result.push(if (r % 2 == 1) {1} else{0});
        carry = if r < 2 {0} else{1};
    }

    if carry != 0 {
        result.push(1);
    }
    result
}

fn mult(mut num: i32, mut num2: i32)->Vec<i32>{
    let mut a = to_binary(num);
    let mut b = to_binary(num2);
    let mut suma = a;
    for i in 0..num2-1{ //B en decimal
        suma = sum(to_binary(num), suma);
    }
    suma
}


fn main() {
    let number = input("Ingresa un numero: ");
    let number2 = input("Ingresa un numero: ");
    
    /*let mut suma = sum(to_binary(number), to_binary(number2));
    suma.reverse();
    println!("{:?}", suma);*/
    
    let mut multi = mult(number, number2);
    multi.reverse();
    println!("{:?}",multi);
}


