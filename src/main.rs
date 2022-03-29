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

fn complementoA2(mut a: Vec<i32>)->Vec<i32>{
    let n = a.len();
    let mut i = n - 1;
    while i>=0{
        if a[i] == 1{
            break;
        }
        i -= 1;
    }
    if i == !0{
        a
    }else{
        let mut k = i - 1;
        while k >= 0{
            if a[k] == 1{
                a[k] = 0;
            }else{
                a[k] = 1;
            }
        }
        a
    }
}

fn add_neg(mut a: i32, mut b: i32)->Vec<i32>{
    let mut total = Vec::new();
    let mut aC;
    let mut bC;
    if a<0{
        aC = complementoA2(to_binary(a.abs()));
    }else{
        aC = to_binary(a.abs());
    }

    if b<0{
        bC = complementoA2(to_binary(b.abs()));
    }else{
        bC = to_binary(b.abs());
    }
    let mut result = Vec::new();
    if (a+b) < 0 {
        result = complementoA2(sum(to_binary(a), to_binary(b)));
        total.push(sum(aC,bC)[-8:])
        total.push("-"+result[-8:])
        total.append("-"+to_dec(result[-8:]))
    }else{
        result=sum(aC,bC)
        total.push(result[-8:])
        total.push(result[-8:])
        total.push(to_dec(result[-8:])))
    }

    total
}

fn main() {
    let mut number= 0;
    let mut number2= 0;
    let mut bin1 = Vec::new();
    let mut bin2 = Vec::new();

    loop{
        let option = input("\n Ingresa una opción: \n 1.Suma \n 2.Resta \n 
        3.Multiplicación \n 4.División \n 5. Suma con signo \n 6. Resta con signo
        \n 7. Multiplicación con signo \n 8. División con signo \n 9.Salir");
        if option > 0 && option < 9{
            number = input("Ingresa un numero: ");
            number2 = input("Ingresa un numero: ");
            bin1 = to_binary(num1);
            bin2 = to_binary(num2);
            println!("{:?}", bin1);
            println!("{:?}\n", bin2);
        }
        if option==1 {
            let sum = add(bin1.clone(), bin2.clone());
            println!("{} + {} = {}\n", num1, num2, to_dec(sum));
        }
        if option==2 {
            let diff = sub(bin1.clone(), bin2.clone());
            println!("{} - {} = {}\n", num1, num2, to_dec(diff));
        }
        if option==3 {
            let multi = mult(num1, num2);
            println!("{} * {} = {}\n", num1, num2, to_dec(multi));
        }
        if option==4 {
            let quo = div(bin1.clone(), bin2.clone());
            println!("{} / {} = {}\n", num1, num2, to_dec(quo));
        }
        if option==5 {
            
        }
        if option==6 {
            
        }
        if option==7 {
            
        }
        if option==8 {
            
        }
        if option==9 {
            
        }
    }
}

