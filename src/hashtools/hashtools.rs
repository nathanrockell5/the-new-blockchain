use sha2::{Sha256, Digest};

pub struct Hashtools{
    
}
impl Hashtools{
    pub fn create_hash(input_string: String) -> String{
        let mut hasher = Sha256::new();
        hasher.update(input_string.as_bytes());
        let result = hasher.finalize();
        // println!("{:x?}", result)
        Self::to_hex_string(&result)    
    }

    pub fn to_hex_string(bytes: &[u8]) -> String {
        let strs: Vec<String> = bytes.iter()
                                     .map(|b| format!("{:02x}", b))
                                     .collect();
        strs.join("")
    }

    pub fn to_byte_array(strs: &String) -> Vec<u8> {
        use hex::FromHex;
        let a: Vec<u8> = Vec::from_hex(strs).expect("Invalid Hex String");
        a
    }

    pub fn combine_hash(hash1: &String, hash2: &String) -> String{
        let h1 = Self::to_byte_array(&hash1.to_string());
        let h2 = Self::to_byte_array(&hash2.to_string());
        let combined_bytes = [h1, h2].concat();
        return Self::create_hash(Self::to_hex_string(combined_bytes.as_ref()));
    }
    
}
