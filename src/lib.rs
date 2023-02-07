use std::{fs::File, io::Read};

pub fn parsestringtonumber() -> Vec<(i32, i32)>{
    let mut file = File::open("data.txt").expect("Unable to open file");
    // i want to read the numbers in data.txt and store them in a vector
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read file");
    let mut x = vec![];
    let mut y = vec![];
    let mut lines = contents.lines();
    let x_line = lines.next().unwrap();
    let y_line = lines.next().unwrap();
    for x_str in x_line.split_whitespace() {
        let x_num = x_str.parse::<i32>().unwrap();
        x.push(x_num);
    }
    for y_str in y_line.split_whitespace() {
        let y_num = y_str.parse::<i32>().unwrap();
        y.push(y_num);
    }
    combine_vectors(x, y)
}


fn combine_vectors(a: Vec<i32>, b: Vec<i32>) -> Vec<(i32, i32)> {
    let mut result = vec![];
    for (a_element, b_element) in a.iter().zip(b.iter()) {
        result.push((*a_element, *b_element));
    }
    result
}
