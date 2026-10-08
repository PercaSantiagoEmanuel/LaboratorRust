fn is_prime(number: i32) -> bool {
    // Daca numarul este mai mic/egal 1 sau este un numar par diferit de 2, nu este prim.
    if number <= 1 || (number != 2 && number % 2 == 0) {
        return false;
    }

    // Daca numarul este 2, este prim.
    if number == 2 {
        return true;
    }

    // count-ul va verifica daca pana la ,,number'' exista un divizor, este 3 pentru ca este primul numar prim dupa 2.
    let mut count = 3;

    while count <= number / count {
        // Daca gasim un divizor nu este prim.
        if number % count == 0 {
            return false;
        }
        count += 2;
    }
    true
}

fn coprime(n: i32, m: i32) -> bool {
    // Cream copii mutabile ale valorilor.
    let mut a = n;
    let mut b = m;

    // Algoritmul lui Euclid.
    while b != 0 {
        let c = b;
        b = a % b;
        a = c;
    }

    a == 1
}

fn beers_99() {
    println!();
    //Cream loop-ul de la 0 la 99 si il inversam.
    for i in (1..=99).rev() {
        //Daca ajungem la ultima bere, scriem un text custom.
        if i == 1 {
            println!(
                "1 bottle of beer on the wall, \n1 bottle of beer. \nTake one down, pass it around, \nNo bottles of beer on the wall."
            );
            return;
        } else {
            //Daca i inca nu este 1, scriem acelasi text si scadem.
            println!("{} bottles of beer on the wall,", i);
            println!("{} bottles of beer.", i);
            println!("Take one down, pass it around,");
            println!("{} bottles of beer on the wall. \n", i - 1);
        }
    }
}

fn main() {
    for i in 1..=100 {
        if is_prime(i) {
            println!("Numarul {} este prim!", i);
        } else {
            println!("Numarul {} nu este prim!", i);
        }
    }

    for i in 0..=100 {
        for j in 0..=100 {
            if coprime(i, j) {
                println!("Numerele {} si {} sunt coprime!", i, j);
            } else {
                println!("Numerele {} si {} nu sunt coprime!", i, j);
            }
        }
    }

    beers_99();
}
