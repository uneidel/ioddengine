
use crate::engine::Engine;
use log::info;
use bitvec::prelude::*;
use crate::utils::format::*;

impl<'a> Engine<'a>{
    pub fn encode_variable(&self, id: &str, param: &str) -> String {
        let v = self.findvariablebyid(id);
        #[allow(unused)] // Some strange "never read"
        let mut hexstring = String::new();
        let input: Vec<&str> = param.split(',').collect();
        info!("Prefilled Vector with zeros");
        match v.datatype.datatype.as_str() {
            "RecordT" => {
                let datalength = v.datatype.bitlength as usize;
                let mut bitvec: BitVec = std::iter::repeat(false)
                .take(datalength)
                .collect();
                for ri in &v.datatype.recorditem {
                    let subindex = ri.subindex as usize;
                    let offset = ri.bit_offset as usize;
                    match ri.datatype.datatype.as_str() {
                        "BooleanT" => {
                            info!("Subindex: {}, Value: {}", subindex, input[subindex - 1]);
                            let val: bool = input[subindex - 1].trim().parse().unwrap();
                            info!("Boolean at pos {}", offset);
                            bitvec.set(offset, val);
                        }
                        "UIntegerT" => {
                            encode_uintegert(
                                &mut bitvec,
                                offset,
                                ri.datatype.bit_length as usize,
                                input[subindex - 1],
                            );
                        }
                        _ => panic!("Not Implemented"),
                    }
                }

                hexstring = Self::convert_to_hex_string(&mut bitvec, datalength);
            }
            "UIntegerT" => {
                let datalength = v.datatype.bitlength as usize;
                let mut bitvec: BitVec = std::iter::repeat(false)
                .take(datalength)
                .collect();
                encode_uintegert(&mut bitvec, 0, datalength, input[0]);
                hexstring = Self::convert_to_hex_string(&mut bitvec, datalength);
            }
            _ => {
                panic!("Not Implemented");
            }
        }
        hexstring
    }

    pub fn convert_to_hex_string(bitvec: &mut BitVec, datalength: usize) -> String {
        let bytes = bitvec.as_raw_slice();
        // Convert bytes to hexadecimal representation
        let mut hexstring = bytes.iter().fold(String::new(), |mut acc, &b| {
            acc.push_str(&format!("{:02x}", b));
            acc
        });

        // do some correction
        let total = (datalength / 8) * 2;
        info!("TotalLength of hexString: {}", total);
        hexstring = pad_left(hexstring, '0', total);
        info!("BitArray: {:?} - HexString: {}", &bitvec, hexstring);
        hexstring
    }

    
}


pub fn encode_uintegert(bitvec: &mut BitVec, offset: usize, bitlength: usize, nbr: &str) {
    let nbr = match nbr.trim().parse::<u32>() {
        Ok(num) => num,
        Err(_) => panic!("Conversion fails..."),
    };        
    info!("UIntegerT from {} {}", offset, bitlength);
    bitvec[offset..bitlength].store(nbr);
}
pub fn encode_integert(bitvec: &mut BitVec, offset: usize, bitlength: usize, nbr: &str) {
    let nbr = match nbr.trim().parse::<i32>() {
        Ok(num) => num,
        Err(e) => panic!("Conversion fails...{}", e),
    };        

    dbg!("IntegerT:{} from Offset {} with Bitlength {}",nbr, offset, bitlength);
    bitvec[offset..bitlength].store::<i32>(nbr);

}

pub fn encode_float32t(bitvec: &mut BitVec, offset: usize, bitlength: usize, nbr: &str) {
    let nbr = match nbr.trim().parse::<f32>() {
        Ok(num) => num,
        Err(e) => panic!("Conversion fails...{}", e),
    };
    dbg!("Encoding Float32T", nbr, "with Offset:", offset, "BitLength:", bitlength);

    if offset + bitlength > bitvec.len() {
        panic!("Invalid bit range");
    }
    bitvec[offset..offset + bitlength].store::<u32>(nbr.to_bits());

    // Print debugging information without format! inside dbg!
    dbg!("Float32T After BitVec:", bitvec);
}

pub fn encode_stringt(bitvec: &mut BitVec, offset: usize, _bitlength: usize,encoding: &str, nbr: &str) {
    // replace special characters
    let fixednbr :String = nbr.chars()
    .map(|ch| match ch {
        '<' => "&lt;".to_string(),
        '>' => "&gt;".to_string(),
        '"' => "&quot;".to_string(),
        '\'' => "&apos;".to_string(),
        '&' => "&amp;".to_string(),
        _ => ch.to_string(),
    })
    .collect();

    match encoding.to_lowercase().as_str() {
        "utf-8" => {
            let butf8 = fixednbr.as_bytes();
            let mut off = offset;
            for b in butf8{
                bitvec[off..off+8].store(*b);
                off+=8;
            }
        },
        "ascii" => {
            let bytes_ascii: Vec<u8> = fixednbr
                .chars()
                .filter(|&c| c as u32 <= 127) // Filter out non-ASCII characters
                .map(|c| c as u8) // Convert char to ASCII bytes
                .collect();
            
            let mut off = offset;
            for b in bytes_ascii{
                bitvec[off..off+8].store(b);
                off+=8;
            }
        }
        _ => panic!("Unsupported encoding"),
    };


}        