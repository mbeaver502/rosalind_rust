use std::{fs::File, io::Read, path::Path};

fn main() {
    let path = Path::new("./datasets/rosalind_ini2_3_dataset.txt");
    let mut file = match File::open(&path) {
        Ok(f) => f,
        Err(e) => panic!("{}", e),
    };

    let mut s = String::new();
    let a: i32;
    let b: i32;

    match file.read_to_string(&mut s) {
        Ok(_) => {
            s = s.trim().to_string();

            let mut split = s.split(" ");

            match split.next() {
                Some(n) => {
                    a = match n.parse::<i32>() {
                        Ok(a) => a,
                        Err(e) => panic!("{}", e),
                    }
                }
                None => todo!(),
            }

            match split.next() {
                Some(n) => {
                    b = match n.parse::<i32>() {
                        Ok(b) => b,
                        Err(e) => panic!("{}", e),
                    }
                }
                None => todo!(),
            }
        }
        Err(e) => panic!("{}", e),
    }

    println!("{} {} = {}", a, b, ini2(a, b));

    let s = ini3("HumptyDumptysatonawallHumptyDumptyhadagreatfallAlltheKingshorsesandalltheKingsmenCouldntputHumptyDumptyinhisplaceagain", 22,27,97,102);
    println!("{}", s);
}

/// https://rosalind.info/problems/ini2/
fn ini2(a: i32, b: i32) -> i32 {
    a.pow(2) + b.pow(2)
}

fn ini3(s: &str, a: usize, b: usize, c: usize, d: usize) -> String {
    let s1 = s.get(a..=b).unwrap();
    let s2 = s.get(c..=d).unwrap();

    let mut s1 = s1.to_owned();
    s1.push_str(" ");
    s1.push_str(s2);

    s1
}
