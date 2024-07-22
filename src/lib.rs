use std::collections::HashMap;

// #[derive(Default)]
pub struct LZW {
    compression_map: HashMap<String, u32>,
    decompression_map: HashMap<u32, String>,
}

impl Default for LZW {
    fn default() -> Self {
        let mut compression_map = HashMap::new();
        let mut decompression_map = HashMap::new();

        (0..255).for_each(|i| {
            compression_map.insert((char::from(i)).to_string(), i as u32);
            decompression_map.insert(i as u32, (char::from(i)).to_string());
        });

        LZW {
            compression_map,
            decompression_map,
        }
    }
}

impl LZW {
    pub fn new(
        compression_map: HashMap<String, u32>,
        decompression_map: HashMap<u32, String>,
    ) -> Self {
        LZW {
            compression_map,
            decompression_map,
        }
    }

    pub fn decompress(&mut self, message: Vec<u32>) -> String {
        let mut last_entry = self.decompression_map.len() as u32;
        let mut ocode = message[0];
        // print!("{:?}", char::from_u32(ocode).unwrap());
        let mut s = String::default();
        let mut res: String = format!("{}", char::from_u32(ocode).unwrap());

        let mut c = char::default();

        message.into_iter().skip(1).for_each(|ncode| {
            // println!("{:?}", ncode);
            // let new = ncode;
            // let sc = format!("{}{}", s, c as char);
            // println!("{:?}", sc);

            match self.decompression_map.get(&ncode) {
                Some(v) => {
                    // println!("{}\t{}\t{}\tY\tx", s, c, sc);
                    // s = format!("{}{}", s, c as char);
                    // println!("{} found: {:?}", ncode, v);
                    s = v.to_string();
                }
                None => {
                    // res.push(*self.compression_map.get(s.as_str()).expect("Table pre-allocated"));
                    // println!("{}\t{}\t{}\tN\t{}", s, c, sc, my_map.get(s.as_str()).unwrap());
                    // last_entry += 1;
                    // self.compression_map.insert(sc, last_entry);
                    // s = (c as char).to_string();
                    s = char::from_u32(ocode).unwrap().to_string();
                    s = format!("{}{}", s, c);
                }
            };
            // print!("{:?}", s);
            // res.push(s.to_string());
            res = format!("{}{}", res, s);
            c = s.chars().nth(0).unwrap();
            last_entry += 1;
            self.decompression_map.insert(
                last_entry,
                format!(
                    "{}{}",
                    self.decompression_map.get(&ocode).unwrap(),
                    c.to_owned()
                ),
            );
            ocode = ncode;
        });

        // res.push(*self.compression_map.get(s.as_str()).expect("Table pre-allocated"));

        res
    }

    pub fn compress(&mut self, message: Vec<u8>) -> Vec<u32> {
        let mut last_entry = self.compression_map.len() as u32;
        let mut s = (message[0] as char).to_string();
        // println!("{:?}", s);

        let mut res: Vec<u32> = Vec::default();

        message
            .into_iter()
            .skip(1)
            // .zip(257..4096)
            .for_each(|c| {
                let sc = format!("{}{}", s, c as char);
                // println!("{:?}", sc);

                match self.compression_map.get(&sc) {
                    Some(_) => {
                        // println!("{}\t{}\t{}\tY\tx", s, c, sc);
                        s = format!("{}{}", s, c as char);
                    }
                    None => {
                        res.push(
                            *self
                                .compression_map
                                .get(s.as_str())
                                .expect("Table pre-allocated"),
                        );
                        // println!("{}\t{}\t{}\tN\t{}", s, c, sc, my_map.get(s.as_str()).unwrap());
                        last_entry += 1;
                        self.compression_map.insert(sc, last_entry);
                        s = (c as char).to_string();
                    }
                };
            });

        res.push(
            *self
                .compression_map
                .get(s.as_str())
                .expect("Table pre-allocated"),
        );

        // println!("{:?}", self.compression_map);

        res
    }

    pub fn calculate_compression_ratio(&self, uncompressed: u32, compressed: u32) -> f64 {
        compressed as f64 / uncompressed as f64
    }
}

#[cfg(test)]
mod tests {
    use super::LZW;

    #[test]
    fn compress_wikipedia_example() {
        let message = Vec::from("TOBEORNOTTOBEORTOBEORNOT");
        let mut thing = LZW::default();
        let result = thing.compress(message);

        let expected_result = vec![
            84, 79, 66, 69, 79, 82, 78, 79, 84, 256, 258, 260, 265, 259, 261, 263,
        ];

        assert_eq!(expected_result, result);
    }

    #[test]
    fn decompress_wikipedia_example() {
        let message = vec![
            84, 79, 66, 69, 79, 82, 78, 79, 84, 256, 258, 260, 265, 259, 261, 263,
        ];
        let mut thing = LZW::default();
        let result = thing.decompress(message);

        let expected_result = String::from("TOBEORNOTTOBEORTOBEORNOT");

        assert_eq!(expected_result, result);
    }

    #[test]
    fn compress_single_char() {
        let message = Vec::from("a");
        let mut thing = LZW::default();
        let result = thing.compress(message);

        let expected_result = vec![97];

        assert_eq!(expected_result, result);
    }

    #[test]
    fn decompress_single_char() {
        let message = vec![97];
        let mut thing = LZW::default();
        let result = thing.decompress(message);

        let expected_result = String::from("a");

        assert_eq!(expected_result, result);
    }
}
