//Adauga un numar n de caractere c la finalul stringului s
fn add_chars_n(mut s: String, c: char, n: i32) -> String {
    let mut i: i32 = 0;
    while i < n {
        s.push(c);
        i += 1;
    }
    s
}

//Modifica s, adaugand c de n ori la nivel de memorie, nu mai este necesar sa returneze s
fn add_chars_n_reference(s: &mut String, c: char, n: i32) {
    let mut i = 0;
    while i < n {
        s.push(c);
        i += 1;
    }
}

//Adauga lui s, n spatii la final
fn add_space(s: &mut String, mut n: i32) {
    while n > 0 {
        s.push(' ');
        n -= 1;
    }
}

//Adauga s1 la finalul lui s caracter cu caracter
fn add_string(s: &mut String, s1: &str) {
    for c in s1.chars() {
        s.push(c);
    }
}

fn add_integer(s: &mut String, mut n: i32) {
    //Daca nr este 0, il adaugam si ne oprim
    if n == 0 {
        s.push('0');
        return;
    }
    //Daca nr este negativ, ii extragem minusul si lucram doar cu pozitivul
    if n < 0 {
        s.push('-');
        n = -n;
    }

    //Pentru stocarea nr in string
    let mut temp = String::new();
    let mut count = 0;

    while n > 0 {
        //Daca inca avem nr si count e divizibil cu 3, punem _
        if count > 0 && count % 3 == 0 {
            temp.push('_');
        }
        //Adaugam ca si ASCII in string
        temp.push(((n % 10) as u8 + b'0') as char);
        n /= 10;
        count += 1;
    }

    //Punem noul string in s
    for c in temp.chars().rev() {
        s.push(c);
    }
}

fn add_float(s: &mut String, val: f32) {
    //Separam valoarea intreaga de zecimale
    let int_p = val as i32;
    let dec_p = ((val - int_p as f32) * 1000.0).round() as i32;

    //Scriem partea intreaga
    add_integer(s, int_p);
    s.push('.');

    //Scriem zecimalele, 3
    if dec_p < 10 {
        s.push('0');
        s.push('0');
    } else if dec_p < 100 {
        s.push('0');
    }

    add_integer(s, dec_p);
}

fn main() {
    //Exercitiul 1
    let mut s = String::from("");
    let mut i = 0;
    while i < 26 {
        let c = (i as u8 + b'a') as char;
        s = add_chars_n(s, c, 26 - i);

        i += 1;
    }

    println!("{}", s);
    //

    println!();

    //Exercitiul 2
    let s = &mut String::from("");
    let mut i = 0;
    while i < 26 {
        let c = (i as u8 + b'a') as char;
        //Apelam direct functia pentru ca s este rescris direct din memorie
        add_chars_n_reference(s, c, 26 - i);
        i += 1;
    }
    println!("{}", s);
    //

    println!();

    //Exercitiul 3
    // Exercitiul 3
    let mut s3 = String::new();

    // Rândul 1
    add_space(&mut s3, 40);
    add_string(&mut s3, "I");
    add_space(&mut s3, 1);
    add_string(&mut s3, "💚\n");

    // Rândul 2
    add_space(&mut s3, 40);
    add_string(&mut s3, "RUST.\n\n");

    // Rândul 3
    add_space(&mut s3, 4);
    add_string(&mut s3, "Most");
    add_space(&mut s3, 12);
    add_string(&mut s3, "crate");
    add_space(&mut s3, 6);
    add_integer(&mut s3, 306437968);
    add_space(&mut s3, 11);
    add_string(&mut s3, "and");
    add_space(&mut s3, 5);
    add_string(&mut s3, "lastest");
    add_space(&mut s3, 9);
    add_string(&mut s3, "is\n");

    // Rândul 4
    add_space(&mut s3, 9);
    add_string(&mut s3, "downloaded");
    add_space(&mut s3, 8);
    add_string(&mut s3, "has");
    add_space(&mut s3, 13);
    add_string(&mut s3, "downloads");
    add_space(&mut s3, 5);
    add_string(&mut s3, "the");
    add_space(&mut s3, 9);
    add_string(&mut s3, "version");
    add_space(&mut s3, 4);
    add_float(&mut s3, 2.038);
    add_string(&mut s3, ".");

    print!("{}", s3);
}
