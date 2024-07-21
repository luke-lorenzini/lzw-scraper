use std::fs;
use std::collections::{HashMap, VecDeque};

fn main() {
    let message = fs::read_to_string("./input_text").expect("Unable to read file");
    // println!("{}", message);
    // let message = "TOBEORNOTTOBEORTOBEORNOT";

    let mut my_map = HashMap::new();
    my_map.insert("A".to_owned(), 65);
    my_map.insert("B".to_owned(), 66);
    my_map.insert("C".to_owned(), 67);
    my_map.insert("D".to_owned(), 68);
    my_map.insert("E".to_owned(), 69);
    my_map.insert("F".to_owned(), 70);
    my_map.insert("G".to_owned(), 71);
    my_map.insert("H".to_owned(), 72);
    my_map.insert("I".to_owned(), 73);
    my_map.insert("J".to_owned(), 74);
    my_map.insert("K".to_owned(), 75);
    my_map.insert("L".to_owned(), 76);
    my_map.insert("M".to_owned(), 77);
    my_map.insert("N".to_owned(), 78);
    my_map.insert("O".to_owned(), 79);
    my_map.insert("P".to_owned(), 80);
    my_map.insert("Q".to_owned(), 81);
    my_map.insert("R".to_owned(), 82);
    my_map.insert("S".to_owned(), 83);
    my_map.insert("T".to_owned(), 84);
    my_map.insert("U".to_owned(), 85);
    my_map.insert("V".to_owned(), 86);
    my_map.insert("W".to_owned(), 87);
    my_map.insert("X".to_owned(), 88);
    my_map.insert("Y".to_owned(), 89);
    my_map.insert("Z".to_owned(), 90);

    let mut last_entry = 256;
    let mut x: VecDeque<_> = message.chars().collect();
    let mut s = String::from(x.pop_front().unwrap());

    let mut res: Vec<usize> = Vec::default();
    
    x
    .into_iter()
    .for_each(|c|
        {
            let sc = format!("{}{}", s, c);
            
            match my_map.get(&sc) {
                Some(_) => {
                        // println!("{}\t{}\t{}\tY\tx", s, c, sc);
                        s = format!("{}{}", s, c);
                    },
                None => {
                    res.push(*my_map.get(s.as_str()).unwrap());
                    // println!("{}\t{}\t{}\tN\t{}", s, c, sc, my_map.get(s.as_str()).unwrap());
                    last_entry += 1;
                    my_map.insert(sc, last_entry);
                    s = String::from(c);
                }
            };
        })
        ;

    res.push(*my_map.get(s.as_str()).unwrap());

    let expected_result = vec![84, 79, 66, 69, 79, 82, 78, 79, 84, 257, 259, 261, 266, 260, 262, 264];
    println!("{:?}", res);

    assert_eq!(expected_result, res);
}
