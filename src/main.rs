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
    let mut suma = to_binary(num);
    for i in 0..num2-1{ //B en decimal
        suma = sum(to_binary(num), suma);
    }
    suma
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

fn sum_neg(mut a: i32, mut b: i32)->Vec<i32>{
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
        
    }

    total
}

fn main() {
    let mut number= 0;
    let mut number2= 0;
    loop{
        let option = input("\n Ingresa una opción: \n 1.Suma \n 2.Resta \n 
        3.Multiplicación \n 4.División \n 5. Suma con signo \n 6. Resta con signo
        \n 7. Multiplicación con signo \n 8. División con signo \n 9.Salir");
        if option > 0 && option < 9{
            number = input("Ingresa un numero: ");
            number2 = input("Ingresa un numero: ");
        }
        if option==1 {
            let mut suma = sum(to_binary(number), to_binary(number2));
            suma.reverse();
            println!("{:?}", suma);
        }
        if option==2 {
            
        }
        if option==3 {
            let mut multi = mult(number, number2);
            multi.reverse();
            println!("{:?}",multi);
        }
        if option==4 {

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


