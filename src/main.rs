use std::io;

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
        println!("{} + {} + {} = ", a[i], b[i], carry);
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
        println!("{}, {} ", res[i], carry);
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

fn main() {
    let num1 = input("Enter number 1");
    let num2 = input("Enter number 2");
    let bin1 = to_binary(num1);
    let bin2 = to_binary(num2);
    println!("{:?}", bin1);
    println!("{:?}\n", bin2);
    let mut sum = add(bin1.clone(), bin2.clone());
    sum.reverse();
    println!("{:?} + {:?} = {:?}\n", bin1, bin2, sum);
}


