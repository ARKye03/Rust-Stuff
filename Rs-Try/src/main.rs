use std::io;
use std::{thread, time};

fn main() {
    println!("Ecuación cuadrática de tipo ax^2 + bx + c\n Dame a, b y c:\n");
    let Sol1: i64;
    let Sol2: i64;
    println!("Valor de a: ");
    let mut p_a = String::new();
    io::stdin().read_line(&mut p_a)
        .expect("Un string papu");
    let a: i64 = match p_a.trim().parse(){
        Ok(nm) => nm,
        Err(_) => panic!("Algo salió mal pibe"),
    }; println!("Valor de b: ");
    let mut p_b = String::new();
    io::stdin().read_line(&mut p_b)
        .expect("Un string papu");
    let b: i64 = match p_b.trim().parse(){
        Ok(nm) => nm,
        Err(_) => panic!("Algo salió mal pibe"),
    }; println!("Valor de c: ");
    let mut p_c = String::new();
    io::stdin().read_line(&mut p_c)
        .expect("Un string papu");
    let c: i64 = match p_c.trim().parse(){
        Ok(nm) => nm,
        Err(_) => panic!("Algo salió mal pibe"),
    };
    let D = ((b*b) - 4 * a * c);
    if D <= 0 { println!("Parece que el discriminante valio verga para los reales entonces, bye...") }

    Sol1 = (-b + i64::pow(D, (1/2))) / 2 * a;
    Sol2 = (-b - i64::pow(D, (1/2))) / 2 * a;

    println!("Las soluciones son X1 = {0} y X2 = {1}", Sol1, Sol2);
    thread::sleep(time::Duration::from_millis(5000));
    }
//Como hallo la raiz cuadrada de un número
