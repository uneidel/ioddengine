use bitvec::prelude::*;
use log::{info, trace};


pub fn convert_slice(input: &[u8]) -> (&[u8], usize) {
    (input, input.len())
}

pub fn hextobytes(hexstr: &str) -> Result<Vec<u8>, String> {
    if hexstr.len() % 2 != 0 {
        return Err("not mul 2".to_string());
    }
    let bytes = hex::decode(hexstr).expect("Failed to decode hex string");
    Ok(bytes)
}

pub fn get_uintegert(bits: &BitVec, offset: usize, length: usize) -> u32 {
   
    info!("IntegerT Offset: {} with Length: {}",offset, length);
    trace!("BitVector: {}", bits);
    let val = &bits[offset..(offset+length)];
    trace!("Taking Vec from {} to {}", offset, offset+length);

    if val.len() > 32 {
        panic!("Bitlength longer than expected.");
    }
    
     let result : u32 = match length{
        1..=16 =>  val.load::<u16>() as u32,
        32 => val.load::<u32>(),
        _ => panic!("Conversion IntegerT fails")
    };
    info!("UIntegerT Raw Result: {}", result);
    result
}
pub fn get_integert(bits: &BitVec, offset: usize, length: usize) -> i32 {
    info!("IntegerT Offset: {} with Length: {}",offset, length);
    info!("BitVector: {}", bits);
    let val = &bits[offset..(offset+length)];
    trace!("Taking Vec from {} to {}", offset, offset+length);
    trace!("val: {}", val);
    if val.len() > 32 {
        panic!("Bitlength longer than expected.");
    }
    
     let result : i32 = match length{
        1..=16 =>  val.load::<i16>() as i32,
        32 => val.load::<i32>(),
        _ => panic!("Conversion IntegerT fails")
    };
    info!("IntegerT Raw Result: {}", result);
    result
}
pub fn get_booleant(bits: &BitVec, offset : usize) -> bool{
    info!("BooleanT");
    bits[offset]
}

pub fn get_float32t(bits: &BitVec, offset : usize) -> f32 {
    info!("Float32T Offset: {} with Length: {}",offset, 32);
    let length = 32;
    info!("Decoding with offset {} and length: {}",offset, length);
    let val = &bits[offset..(offset+length)];
    if val.len() > 32 {
        panic!("Bitlength longer than expected.");
    }
    let v = val.load::<u32>();
    f32::from_bits(v)
}

pub fn get_stringt(_bits: &BitVec, offset : usize, length: usize, encoding: &str) -> String{
    info!("StringT Offset: {}, Length:{}, encoding:{}", offset, length, encoding);

    todo!("Implement me");
    /*let val = &bits[offset..(offset+length)];    
    let res = val.
    let x = std::str::from_utf8(res).unwrap();
    */
}


