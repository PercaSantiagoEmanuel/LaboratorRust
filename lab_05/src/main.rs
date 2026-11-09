use serde_derive::Deserialize;
use std::fs;

// P1
//debug for println, clone to be able to make copies, deserialize for json
#[derive(Debug, Clone, Deserialize)]
struct Student {
    name: String,
    phone: String,
    age: u32,
}

fn oldest_youngest(content: &str) -> Option<(Student, Student)> {
    // First we create 2 phantom students as youngest and oldest
    let mut youngest: Option<Student> = None;
    let mut oldest: Option<Student> = None;

    // We start reading the file
    for line in content.lines() {
        if line.trim().is_empty() {
            continue; // ignores empty lines
        }

        // Selecting text and numbers using ","" as a separator
        let item: Vec<&str> = line.split(',').collect();

        if item.len() == 3 {
            // If parse fails, we jump the line
            let age: u32 = match item[2].trim().parse() {
                Ok(val) => val,
                Err(_) => continue,
            };

            let student_curent = Student {
                name: item[0].trim().to_string(),
                phone: item[1].trim().to_string(),
                age,
            };

            // Compare current student with youngest and oldest
            match &youngest {
                None => youngest = Some(student_curent.clone()),
                Some(y) => {
                    if student_curent.age < y.age {
                        youngest = Some(student_curent.clone());
                    }
                }
            }

            match &oldest {
                None => oldest = Some(student_curent.clone()),
                Some(o) => {
                    if student_curent.age > o.age {
                        oldest = Some(student_curent.clone());
                    }
                }
            }
        }
    }

    // Returnăm perechea doar dacă ambele opțiuni conțin valori
    match (youngest, oldest) {
        (Some(y), Some(o)) => Some((y, o)),
        _ => None,
    }
}

// P2
//Creating an empty matrix
struct Canvas {
    grid: Vec<Vec<char>>,
}

//Creates a matrix with 55 rows each with 100 blank characters
fn new_canvas() -> Canvas {
    Canvas {
        grid: vec![vec![' '; 100]; 55],
    }
}

//Takes the y, x coordinates and paints the u8 number as the ascii character there
fn set_pixels(canvas: &mut Canvas, pixels: &[(usize, usize, u8)]) {
    for &(y, x, ascii_value) in pixels {
        // A safety check to be within bounds
        if y < 55 && x < 100 {
            canvas.grid[y][x] = ascii_value as char;
        }
    }
}

//Prints line by line the canvas
fn print(canvas: Canvas) {
    for row in canvas.grid {
        let line_text: String = row.into_iter().collect();
        println!("{}", line_text);
    }
}

// P3
// Copied P1 and modified it for json
fn oldest_youngest_json(content: &str) -> Option<(Student, Student)> {
    let mut youngest: Option<Student> = None;
    let mut oldest: Option<Student> = None;

    for line in content.lines() {
        let line_trimmed = line.trim();
        if line_trimmed.is_empty() {
            continue; // Ignore the blank lines
        }

        // If the json is invalid we jump the line
        let student_curent: Student = match serde_json::from_str(line_trimmed) {
            Ok(s) => s,
            Err(_) => continue,
        };

        match &youngest {
            None => youngest = Some(student_curent.clone()),
            Some(y) => {
                if student_curent.age < y.age {
                    youngest = Some(student_curent.clone());
                }
            }
        }

        match &oldest {
            None => oldest = Some(student_curent.clone()),
            Some(o) => {
                if student_curent.age > o.age {
                    oldest = Some(student_curent.clone());
                }
            }
        }
    }

    match (youngest, oldest) {
        (Some(y), Some(o)) => Some((y, o)),
        _ => None,
    }
}

fn main() {
    // P1
    match fs::read_to_string("P1.txt") {
        Ok(content) => match oldest_youngest(&content) {
            Some((young, old)) => {
                println!(
                    "Youngest student: {}, age: {}, Phone no.: {}",
                    young.name, young.age, young.phone
                );
                println!(
                    "Oldest student: {}, age: {}, Phone no.: {}",
                    old.name, old.age, old.phone
                );
            }
            None => {
                println!("Erorr: File empty or invalid students!");
            }
        },
        Err(_) => println!("Can't open file!"),
    }

    // P2
    let mut canvas = new_canvas();
    let c = &mut canvas;

    set_pixels(c, &[(4, 25, 124), (3, 33, 124), (2, 24, 95), (4, 3, 95)]);
    set_pixels(c, &[(7, 2, 95), (4, 21, 124), (5, 16, 95)]);
    set_pixels(c, &[(4, 41, 124), (7, 1, 124), (5, 8, 92)]);
    set_pixels(c, &[(1, 31, 40), (2, 3, 95), (2, 41, 124)]);
    set_pixels(
        c,
        &[
            (2, 16, 95),
            (5, 35, 92),
            (6, 3, 95),
            (2, 11, 95),
            (5, 3, 95),
        ],
    );
    set_pixels(
        c,
        &[
            (2, 38, 95),
            (4, 9, 40),
            (3, 41, 124),
            (2, 37, 95),
            (2, 25, 124),
        ],
    );
    set_pixels(
        c,
        &[
            (5, 27, 124),
            (2, 27, 124),
            (4, 0, 124),
            (3, 35, 47),
            (2, 18, 95),
        ],
    );
    set_pixels(c, &[(4, 13, 124), (4, 37, 95), (4, 16, 40), (3, 6, 124)]);
    set_pixels(c, &[(7, 32, 47), (4, 20, 124), (5, 11, 95), (5, 42, 95)]);
    set_pixels(c, &[(5, 15, 92), (4, 34, 124), (4, 45, 41), (5, 24, 95)]);
    set_pixels(c, &[(4, 2, 40), (7, 3, 95), (2, 44, 95)]);
    set_pixels(
        c,
        &[
            (6, 30, 95),
            (5, 45, 95),
            (4, 31, 124),
            (4, 7, 124),
            (3, 43, 39),
        ],
    );
    set_pixels(c, &[(5, 17, 95), (1, 27, 124), (2, 5, 95)]);
    set_pixels(
        c,
        &[
            (3, 44, 95),
            (3, 19, 92),
            (5, 23, 95),
            (3, 8, 47),
            (2, 10, 95),
        ],
    );
    set_pixels(c, &[(6, 6, 124), (5, 19, 47), (3, 24, 95), (3, 27, 124)]);
    set_pixels(
        c,
        &[
            (3, 10, 95),
            (4, 44, 95),
            (2, 9, 95),
            (0, 32, 95),
            (5, 2, 95),
        ],
    );
    set_pixels(c, &[(6, 2, 95), (7, 31, 95), (1, 25, 124), (2, 36, 95)]);
    set_pixels(
        c,
        &[
            (3, 46, 92),
            (5, 25, 44),
            (1, 43, 124),
            (5, 46, 47),
            (3, 15, 47),
        ],
    );
    set_pixels(c, &[(4, 17, 95), (2, 23, 95), (3, 39, 92)]);
    set_pixels(c, &[(4, 47, 124), (2, 45, 95), (3, 37, 95)]);
    set_pixels(
        c,
        &[
            (5, 44, 95),
            (2, 2, 95),
            (5, 10, 95),
            (5, 9, 95),
            (4, 43, 124),
        ],
    );
    set_pixels(c, &[(4, 38, 41), (2, 17, 95), (0, 26, 95)]);
    set_pixels(c, &[(4, 18, 41), (7, 5, 47), (5, 41, 124), (5, 33, 124)]);
    set_pixels(c, &[(5, 12, 47), (5, 22, 92), (6, 33, 124), (5, 31, 124)]);
    set_pixels(
        c,
        &[
            (4, 40, 124),
            (3, 3, 95),
            (4, 4, 124),
            (6, 31, 47),
            (3, 4, 96),
        ],
    );
    set_pixels(c, &[(0, 42, 95), (5, 18, 95), (4, 27, 124)]);
    set_pixels(
        c,
        &[
            (3, 12, 92),
            (2, 32, 95),
            (5, 37, 95),
            (5, 26, 95),
            (5, 39, 47),
        ],
    );
    set_pixels(c, &[(3, 25, 96), (4, 14, 124), (4, 33, 124), (3, 1, 47)]);
    set_pixels(
        c,
        &[
            (5, 36, 95),
            (7, 30, 95),
            (6, 4, 47),
            (4, 24, 95),
            (1, 32, 95),
        ],
    );
    set_pixels(c, &[(3, 22, 47), (4, 23, 40), (5, 6, 124)]);
    set_pixels(c, &[(1, 33, 41), (1, 41, 124), (7, 29, 124)]);
    set_pixels(c, &[(4, 6, 124), (5, 38, 95), (3, 31, 124), (7, 4, 95)]);
    set_pixels(c, &[(4, 11, 41), (4, 10, 95), (5, 1, 92)]);
    set_pixels(c, &[(2, 43, 124), (3, 17, 95), (5, 4, 44), (4, 36, 40)]);
    set_pixels(c, &[(5, 43, 46)]);

    print(canvas);

    // P3
    match fs::read_to_string("P3.txt") {
        Ok(content) => match oldest_youngest_json(&content) {
            Some((young, old)) => {
                println!(
                    "Youngest student: {}, age: {}, Phone no.: {}",
                    young.name, young.age, young.phone
                );
                println!(
                    "Oldest student: {}, age: {}, Phone no.: {}",
                    old.name, old.age, old.phone
                );
            }
            None => {
                println!("Erorr: FIle empty or invalid students!");
            }
        },
        Err(_) => println!("Can't open file!"),
    }
}
