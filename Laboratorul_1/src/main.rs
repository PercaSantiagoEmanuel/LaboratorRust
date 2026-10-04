
// Algoritm pnetru numar prim
fn is_prime(number : i32){

    // Daca numarul este mai mic ca 0, daca este 1 sau un numar par diferit de 2, nu este prim.
    if number < 1 || number == 1 || (number!=2 && number%2==0){
        println! ("Numarul {} nu este prim!", number);
        return;
    }
    // Daca numarul este 2 este prim.
    if number==2{
        println!("Numarul {} este prim!", number); return;
    }

    // Cream un count care va trebui sa ajunga pana la numarul original, initial este 3 deoarece primul numar prim dupa 2 este 3.
    let mut count = 3;

    // Loop-ul trece prin toate numerele impare de la 3 pana la numarul nostru, daca nu se gaseste niciun divizor este prim, altfel nu este prim.
    while count < number{
        if number % count == 0{
            println!("Numarul {} nu este prim!", number);
            return;
        }
        count+=2;
    }
    println!("Numarul {} este prim!", number); return;

}

// Algoritm pentru coprime.
fn coprime( n:i32, m:i32) -> bool {
    //Cream niste copii pe care le putem modifica.
    let mut a = n;
    let mut b = m;

    //In cazul in care a este mai mare ca b, aplicam Algoritmul lui Euclid prin impartiri incepand de la a.
    //Altfel, daca b este mai mare, incepem de la b.
    if a > b{
        while b!=0{
            let rest = a % b;
            a = b;
            b = rest;
        }
        if a == 1 { 
            println!("Numerele {} si {} sunt coprime!", n, m); return true;
        } else {
            return true;
        }
    }
    else{
        while a!=0{
            let rest = b % a;
            b = a;
            a = rest;
        }

        //Doua numere sunt coprime daca au cmmdc = 1, asa ca, daca b este 1 atunci sunt coprime.
        if b == 1 { 
            println!("Numerele {} si {} sunt coprime!", n, m); return true;
        } else {
            return false;
        } 
    }
    
}

fn beers_99(){
    println!();
    //Cream loop-ul de la 0 la 99 si il inversam.
    for i in (0..=99).rev(){
        //Daca ajungem la ultima bere, scriem un text custom.
        if i == 1{
            println!("1 bottle of beer on the wall, \n 1 bottle of beer. \n Take one down, pass it around, \n No bottles of beer on the wall."); return;
        }

        //Daca i in ca nu este 1, scriem acelasi text si scadem.
        println!("{} bottles of beer on the wall,", i);
        println!("{} bottles of beer.", i);
        println!("Take one down, pass it around,");
        println!("{} bottles of beer on the wall. \n", i-1);
    }
    return;
}


fn main(){
    // Loop-ul de la 0 la 100 pentru a verifica fiecare numar daca este prim.
    let mut count = 0;
    while count < 101{
        is_prime(count);
        count+=1;
    }

    println!(); println!();

    //Verificam toate perechile de 0 la 100 daca sunt coprime.
    for i in 0..=100{
        for j in 0..=100{
            coprime(i,j);
        }
    }

    //Singing the beer problem.
    beers_99();
}