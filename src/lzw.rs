use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

pub struct LZW;

impl Default for LZW {
    fn default() -> Self {
        LZW
    }
}

impl LZW {
    pub fn new() -> Self {
        LZW
    }

    // #[inline]
    pub fn decompress(
        &mut self,
        message: Vec<u32>,
        decompression_map: &mut HashMap<u32, String>,
    ) -> String {
        let mut last_entry = decompression_map.len() as u32;
        let mut ocode = message[0];
        let mut s = String::default();
        let mut res: String = format!("{}", char::from_u32(ocode).unwrap());

        let mut c = char::default();

        message.into_iter().skip(1).for_each(|ncode| {
            match decompression_map.get(&ncode) {
                Some(v) => {
                    s = v.to_string();
                }
                None => {
                    s = char::from_u32(ocode).unwrap().to_string();
                    s = format!("{}{}", s, c);
                }
            };
            res = format!("{}{}", res, s);
            c = s.chars().nth(0).unwrap();
            last_entry += 1;
            decompression_map.insert(
                last_entry,
                format!("{}{}", decompression_map.get(&ocode).unwrap(), c.to_owned()),
            );
            ocode = ncode;
        });

        res
    }

    #[inline]
    pub fn compress(
        &self,
        message: &[u8],
        compression_map: Arc<Mutex<std::collections::HashMap<String, u32>>>,
    ) -> Vec<u32> {
        let mut compression_map = compression_map.lock().unwrap();
        let mut last_entry = compression_map.len() as u32;
        let mut s = (message[0] as char).to_string();
        let mut res: Vec<u32> = Vec::default();

        message.iter().skip(1).for_each(|c| {
            let sc = format!("{}{}", s, *c as char);
            // let sc = s.to_owned() + &(*c as char).to_string();

            match compression_map.get(&sc) {
                Some(_) => {
                    s = format!("{}{}", s, *c as char);
                    // s = s.to_owned() + &(*c as char).to_string();
                }
                None => {
                    res.push(
                        *compression_map
                            .get(s.as_str())
                            .expect("Table pre-allocated"),
                    );
                    last_entry += 1;
                    compression_map.insert(sc, last_entry);
                    s = (*c as char).to_string();
                }
            };
        });

        res.push(
            *compression_map
                .get(s.as_str())
                .expect("Table pre-allocated"),
        );

        res
    }
}
