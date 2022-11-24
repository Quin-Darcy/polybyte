#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_upper_case_globals)]


// Represents the two bytes of the irreducible polynomial
// which generates the ideal (P)_i in the quotient ring Z_2[X] / (P)_i
// that all PolyBytes are defined in.
const P: [u8; 2] = [0x01, 0x1b]; 

#[derive(Debug)]
pub struct PolyByte {
    pub byte: u8,
}

impl PolyByte {
    // Look up table for all multiplicative inverses in Z_2[X] / (P)_i
    const INVERSES: [u8; 255] = [1, 141, 246, 203, 82, 123, 209, 232, 79, 41, 
                                192, 176, 225, 229, 199, 116, 180, 170, 75, 153, 
                                43, 96, 95, 88, 63, 253, 204, 255, 64, 238, 178, 
                                58, 110, 90, 241, 85, 77, 168, 201, 193, 10, 152, 
                                21, 48, 68, 162, 194, 44, 69, 146, 108, 243, 57, 
                                102, 66, 242, 53, 32, 111, 119, 187, 89, 25, 29, 
                                254, 55, 103, 45, 49, 245, 105, 167, 100, 171, 19, 
                                84, 37, 233, 9, 237, 92, 5, 202, 76, 36, 135, 191, 
                                24, 62, 34, 240, 81, 236, 97, 23, 22, 94, 175, 211, 
                                73, 166, 54, 67, 244, 71, 145, 223, 51, 147, 33, 59, 
                                121, 183, 151, 133, 16, 181, 186, 60, 182, 112, 208, 
                                6, 161, 250, 129, 130, 131, 126, 127, 128, 150, 115, 
                                190, 86, 155, 158, 149, 217, 247, 2, 185, 164, 222, 
                                106, 50, 109, 216, 138, 132, 114, 42, 20, 159, 136, 
                                249, 220, 137, 154, 251, 124, 46, 195, 143, 184, 101, 
                                72, 38, 200, 18, 74, 206, 231, 210, 98, 12, 224, 31, 
                                239, 17, 117, 120, 113, 165, 142, 118, 61, 189, 188, 
                                134, 87, 11, 40, 47, 163, 218, 212, 228, 15, 169, 39, 
                                83, 4, 27, 252, 172, 230, 122, 7, 174, 99, 197, 219, 
                                226, 234, 148, 139, 196, 213, 157, 248, 144, 107, 177, 
                                13, 214, 235, 198, 14, 207, 173, 8, 78, 215, 227, 93, 
                                80, 30, 179, 91, 35, 56, 52, 104, 70, 3, 140, 221, 156, 
                                125, 160, 205, 26, 65, 28];
    pub fn new() -> Self {
        PolyByte {
            byte: 0x00,
        }
    }

    pub fn from_byte(b: u8) -> Self {
        PolyByte {
            byte: b,
        }
    }

    pub fn clone(&mut self) -> PolyByte {
        PolyByte {
            byte: self.byte,
        }
    }

    pub fn add(&mut self, b: PolyByte) {
        self.byte = self.byte ^ b.byte
    }

    pub fn sum(b1: &PolyByte, b2: &PolyByte) -> PolyByte {
        PolyByte {
            byte: b1.byte ^ b2.byte,
        }
    }

    pub fn xtimes(&mut self) {
        if self.byte >= 128 {
            self.byte = self.byte << P[0];
            self.byte = self.byte ^ P[1];
        } else {
            self.byte = self.byte << P[0];
        }
    }

    pub fn mult(&mut self, b: &mut PolyByte) {
        let bin: [u8; 8] = byte_to_bin(self.byte);
        self.byte = 0;
        let mut exp: u8;
        let mut mult_val: PolyByte;

        for i in 0..8 {
            if bin[i] == 1 {
                mult_val = b.clone();
                exp = (8-i-1) as u8;
                for j in 0..exp {
                    mult_val.xtimes();            
                } 
                self.add(mult_val);
            }
        }
    }

    pub fn prod(b1: &PolyByte, b2: &mut PolyByte) -> PolyByte {
        let bin: [u8; 8] = byte_to_bin(b1.byte);
        let mut exp: u8;
        let mut mult_val: PolyByte;
        let mut new_polybyte: PolyByte = PolyByte::new();

        for i in 0..8 {
            if bin[i] == 1 {
                mult_val = b2.clone();
                exp = (8-i-1) as u8;
                for j in 0..exp {
                    mult_val.xtimes(); 
                }
                new_polybyte.add(mult_val);
            }
        }
        new_polybyte
    }

    pub fn pow(&mut self, n: u32) {
        let mut base: PolyByte = PolyByte::from_byte(self.byte);
        for _ in 0..n {
            self.mult(&mut base);
        }
    }
    
    pub fn mult_inv(&mut self) -> PolyByte {
        let mut b: u8;
        if self.byte == 0_u8 {
            b = 0_u8;
        } else {
            b = PolyByte::INVERSES[(self.byte-1) as usize];
        }
        PolyByte {
            byte: b,
        }
    }
}

pub struct PolyWord {
    pub word: u32,
}

impl PolyWord {
    pub fn new() -> PolyWord {
        PolyWord {
            word: 0_u32, 
        }
    }

    pub fn from_word(w: u32) -> PolyWord {
        PolyWord {
            word: w,
        }
    }

    pub fn from_bytes(b: [u8; 4]) -> PolyWord {
        PolyWord {
            word: u32::from_be_bytes(b),
        }
    }

    pub fn add(&mut self, w: PolyWord) {
        let mut summed_bytes: [u8; 4] = [0_u8; 4]; 
        let mut a: PolyByte;
        let mut b: PolyByte;
        for i in 0..4 {
            a = PolyByte::from_byte(self.word.to_be_bytes()[i]);
            b = PolyByte::from_byte(w.word.to_be_bytes()[i]);
            a.add(b);
            summed_bytes[i] = a.byte; 
        }
        self.word = u32::from_be_bytes(summed_bytes);
    }

    pub fn mult(&mut self, w: &PolyWord) {
        let b1: [u8; 4] = self.word.to_be_bytes();
        let b2: [u8; 4] = w.word.to_be_bytes();
        let mult_mat: [[u8; 4]; 4] = [[b1[0], b1[1], b1[2], b1[3]],
                                      [b1[3], b1[0], b1[1], b1[2]],
                                      [b1[2], b1[3], b1[0], b1[1]],
                                      [b1[1], b1[2], b1[3], b1[0]]];
        
        let mut new_word: [u8; 4] = [0_u8; 4];
        let mut pb1: PolyByte;
        let mut pb2: PolyByte;
        let mut pb3: PolyByte;
        for i in 0..4 {
            let mut pb1: PolyByte = PolyByte::from_byte(new_word[i]);
            for j in 0..4 {
                let mut pb2: PolyByte = PolyByte::from_byte(mult_mat[j][i]);
                let mut pb3: PolyByte = PolyByte::from_byte(b2[j]);
                pb1.add(PolyByte::prod(&pb2, &mut pb3));
            }
            new_word[i] = pb1.byte;
        }
        self.word = u32::from_be_bytes(new_word);
    }
}

pub fn byte_to_bin(b: u8) -> [u8; 8] {
    let mut t: u8 = 1;
    let mut n: u8 = b;
    let mut index: usize = 0;
    let mut bin_rep: [u8; 8]= [0_u8; 8];
                                   
    for _ in 0..8 {
        if n == 0 { break; }
        for _ in 0..8 {
            if t == 0 { break;  }
            index += 1;
            t = (n >> index) & 0x0f;
        }
        n = n-2_u8.pow((index-1) as u32);
        bin_rep[8-index] = 1;
        t = 1;
        index = 0;
    }
    bin_rep
}

pub fn bin_to_byte(bin_rep:  [u8; 8]) -> u8 {
    let mut dec_rep: u8 = 0;
                            
    for i in 0..8 {
        dec_rep += bin_rep[8-1-i] * 2_u8.pow(i as u32); 
    }
    dec_rep
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut p: PolyByte = PolyByte::from_byte(0xa4);
        let mut q: PolyByte = PolyByte::from_byte(0x39);
        let mut w: PolyWord = PolyWord::from_word(0xcd435f17);
        let mut v: PolyWord = PolyWord::from_word(0x55de3faa);
        p.add(q);
        w.add(v);
        assert_eq!(p.byte, 0x9d);
        assert_eq!(w.word, 0x989d60bd);
    }

    #[test]
    fn test_sum() {
        let mut p: PolyByte = PolyByte::from_byte(0xa4);
        let mut q: PolyByte = PolyByte::from_byte(0x39);
        assert_eq!(PolyByte::sum(&p, &q).byte, 0x9d);
    }

    #[test]
    fn test_mult() {
        let mut p: PolyByte = PolyByte::from_byte(0x57);
        let mut q: PolyByte = PolyByte::from_byte(0x13);
        //let mut w: PolyWord = PolyWord::from_word(0x12121212);
        //let mut v: PolyWord = PolyWord::from_word(0x21212121);
        p.mult(&mut q);
        //w.mult(v);
        assert_eq!(p.byte, 0xfe);
        //assert_eq!(w.word, 0x13f11f1d);
    }

    #[test]
    fn test_prod() {
        let mut p: PolyByte = PolyByte::from_byte(0x57);
        let mut q: PolyByte = PolyByte::from_byte(0x13);
        assert_eq!(PolyByte::prod(&p, &mut q).byte, 0xfe);
    }

    #[test]
    fn test_inverse() {
        let mut p: PolyByte = PolyByte::from_byte(0x31);
        let mut q: PolyByte = p.mult_inv();
        assert_eq!(PolyByte::prod(&p, &mut q).byte, 0x01);
    }
}

