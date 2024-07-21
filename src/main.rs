use std::collections::{HashMap, VecDeque};

fn main() {
    let message = "TOBEORNOTTOBEORTOBEORNOT";

    let mut my_map = HashMap::new();
    my_map.insert("A".to_owned(), 1);
    my_map.insert("B".to_owned(), 2);
    my_map.insert("C".to_owned(), 3);
    my_map.insert("D".to_owned(), 4);
    my_map.insert("E".to_owned(), 5);
    my_map.insert("F".to_owned(), 6);
    my_map.insert("G".to_owned(), 7);
    my_map.insert("H".to_owned(), 8);
    my_map.insert("I".to_owned(), 9);
    my_map.insert("J".to_owned(), 10);
    my_map.insert("K".to_owned(), 11);
    my_map.insert("L".to_owned(), 12);
    my_map.insert("M".to_owned(), 13);
    my_map.insert("N".to_owned(), 14);
    my_map.insert("O".to_owned(), 15);
    my_map.insert("P".to_owned(), 16);
    my_map.insert("Q".to_owned(), 17);
    my_map.insert("R".to_owned(), 18);
    my_map.insert("S".to_owned(), 19);
    my_map.insert("T".to_owned(), 20);
    my_map.insert("U".to_owned(), 21);
    my_map.insert("V".to_owned(), 22);
    my_map.insert("W".to_owned(), 23);
    my_map.insert("X".to_owned(), 24);
    my_map.insert("Y".to_owned(), 25);
    my_map.insert("Z".to_owned(), 26);

    let mut last_entry = my_map.len();
    let mut x: VecDeque<_> = message.chars().collect();
    let mut s: String = String::from(x.pop_front().unwrap());

    let mut res: Vec<usize> = Vec::default();
    
    for c in x {
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
    }

    res.push(*my_map.get(s.as_str()).unwrap());

    let expected_result = vec![20, 15, 2, 5, 15, 18, 14, 15, 20, 27, 29, 31, 36, 30, 32, 34];
    println!("{:?}", res);

    assert_eq!(expected_result, res);
}
