// Ex. 1
fn is_prime(x: u16) -> bool {
    if x <= 1 || (x.is_multiple_of(2) && x != 2) {
        return false;
    }
    if x == 2 {
        return true;
    }
    //We start from 3 because it is the first prime number after 2
    let mut i = 3;
    //Checking numbers till we hit the square root of x
    while i <= x / i {
        if x & i == 0 {
            return false;
        }
        i += 2;
    }

    true
}

fn next_prime(x: u16) -> Option<u16> {
    //If we hit the top, we return NONE
    if x == u16::MAX {
        return None;
    }

    //Searching for the next prime number starting from x+1
    ((x + 1)..=u16::MAX).find(|&i| is_prime(i))
}

//Ex 2
fn can_fail_add(a: u32, b: u32) -> u32 {
    match a.checked_add(b) {
        Some(result) => result,
        None => panic!("Addition Overflowed u32"),
    }
}

fn can_fail_mul(a: u32, b: u32) -> u32 {
    match a.checked_mul(b) {
        Some(result) => result,
        None => panic!("Multiplication Overflowed u32"),
    }
}

//Ex 3
#[derive(Debug)]
enum Error {
    Overflow,
}

fn can_fail_add_res(a: u32, b: u32) -> Result<u32, Error> {
    //If ok, some, else Overflow
    a.checked_add(b).ok_or(Error::Overflow)
}

fn can_fail_mul_res(a: u32, b: u32) -> Result<u32, Error> {
    //If ok, some, else Overflow
    a.checked_mul(b).ok_or(Error::Overflow)
}

//This function does this operation: (a*b)+c
fn test_mul_and_add(a: u32, b: u32, c: u32) -> Result<u32, Error> {
    let product: u32 = can_fail_mul_res(a, b)?;
    let result: u32 = can_fail_add_res(product, c)?;
    Ok(result)
}

//Ex 4
enum CharError {
    NotASCII,
    NotDigit,
    NotBase16,
    NotLetter,
    NotPrintable,
}

fn to_uppercase(c: char) -> Result<char, CharError> {
    if c.is_alphabetic() {
        Ok(c.to_ascii_uppercase())
    } else {
        Err(CharError::NotLetter)
    }
}

fn to_lowercase(c: char) -> Result<char, CharError> {
    if c.is_alphabetic() {
        Ok(c.to_ascii_lowercase())
    } else {
        Err(CharError::NotLetter)
    }
}

fn print_char(c: char) -> Result<(), CharError> {
    if c.is_control() {
        Err(CharError::NotPrintable)
    } else {
        println!("Caracther: {c}");
        Ok(())
    }
}

fn char_to_number(c: char) -> Result<u32, CharError> {
    if !c.is_ascii() {
        return Err(CharError::NotASCII);
    }

    if c.is_ascii_digit() {
        let number = c as u32 - '0' as u32;
        Ok(number)
    } else {
        Err(CharError::NotDigit)
    }
}

fn char_to_number_hex(c: char) -> Result<u32, CharError> {
    if !c.is_ascii() {
        return Err(CharError::NotASCII);
    }

    if c.is_ascii_hexdigit() {
        match c.to_digit(16) {
            Some(val) => Ok(val),
            None => Err(CharError::NotBase16),
        }
    } else {
        Err(CharError::NotBase16)
    }
}

fn print_error(err: CharError) {
    match err {
        CharError::NotASCII => println!("Erorr: NotASCII!"),
        CharError::NotDigit => println!("Erorr: NotDigit!"),
        CharError::NotBase16 => println!("Erorr: NotBase16!"),
        CharError::NotLetter => println!("Erorr: NotLetter!"),
        CharError::NotPrintable => println!("Erorr: NotPrintable!"),
    }
}

//Ex 5

// Option: Search for a student's score
fn get_score(name: &str) -> Option<u32> {
    if name == "Alex" { Some(90) } else { None }
}

// Result: Check if a score is passing
fn check_pass(score: u32) -> Result<&'static str, &'static str> {
    if score >= 50 {
        Ok("Passed!")
    } else {
        Err("Failed!")
    }
}

fn main() {
    //Number 1
    let mut x = 20000;
    while let Some(prime) = next_prime(x) {
        println!("{}", prime);
        x = prime;
    }

    //Number 2
    let sum1 = can_fail_add(100, 200);
    println!("Sum1={}", sum1);

    let mul1 = can_fail_mul(10, 20);
    println!("Mul1={}", mul1);

    //Testing panic!
    //let big_number = u32::MAX;
    //let _sum2 = can_fail_add(big_number, 1);

    //Number 3
    //Success case: (10 * 20) + 5 = 205
    match test_mul_and_add(10, 20, 5) {
        Ok(val) => println!("Result: {val}"),
        Err(err) => println!("Error: {err:?}"),
    }

    //Overflow case: (u32::MAX * 2) + 5
    match test_mul_and_add(u32::MAX, 2, 5) {
        Ok(val) => println!("Result: {val}"),
        Err(err) => println!("Error: {err:?}"),
    }

    //Number 4
    // to_uppercase
    if let Ok(c) = to_uppercase('a') {
        println!("to_uppercase('a') -> {c}");
    }

    // to_lowercase
    if let Ok(c) = to_lowercase('B') {
        println!("to_lowercase('B') -> {c}");
    }

    // print_char
    let _ = print_char('Z');

    //char_to_number
    if let Ok(n) = char_to_number('7') {
        println!("char_to_number('7') -> {n}");
    }

    // char_to_number_hex
    if let Ok(n) = char_to_number_hex('f') {
        println!("char_to_number_hex('f') -> {n}");
    }

    // Erorr 1: NotLetter
    if let Err(err) = to_uppercase('5') {
        print_error(err);
    }

    // Erorr 2: NotPrintable
    if let Err(err) = print_char('\0') {
        print_error(err);
    }

    // Erorr 3: NotDigit
    if let Err(err) = char_to_number('a') {
        print_error(err);
    }

    // Erorr 4: NotBase16
    if let Err(err) = char_to_number_hex('g') {
        print_error(err);
    }

    // Erorr 5: NotASCII
    if let Err(err) = char_to_number('ș') {
        print_error(err);
    }

    //Number 5
    // Testing Option (returns Some or None)
    println!("{:?}", get_score("Alex"));
    println!("{:?}", get_score("Maria"));

    // Testing Result (returns Ok or Err)
    println!("{:?}", check_pass(90));
    println!("{:?}", check_pass(30));
}
