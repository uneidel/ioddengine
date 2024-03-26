// Unit tests
#[cfg(test)]
mod tests {
    use bitvec::prelude::*;
    use super::super::decoder::*;
    use super::super::encoder::*;
   

    #[test]    
    fn test_get_float1() {
        let mut bitvec: BitVec = std::iter::repeat(false).take(32).collect();
        encode_float32t(&mut bitvec, 0, 32, "281.25");
        dbg!(format!("Float32T Final BitVec: {}", bitvec));
        let decoded = get_float32t(&bitvec, 0);
        println!("Decoded Value: {}", decoded);
        assert_eq!(decoded, 281.25);
    }
    #[test]    
    fn test_get_float1_variable_offset() {
        let mut bitvec: BitVec = std::iter::repeat(false).take(48).collect();
        encode_float32t(&mut bitvec, 16, 32, "281.25");
        //dbg!(format!("Float32T Final BitVec: {}", bitvec));
        let decoded = get_float32t(&bitvec, 16);
        println!("Decoded Value: {}", decoded);
        assert_eq!(decoded, 281.25);
    }

    #[test]    
    fn test_negative_float1_variable_offset() {
        let mut bitvec: BitVec = std::iter::repeat(false).take(48).collect();
        encode_float32t(&mut bitvec, 16, 32, "-281.25");
        //dbg!(format!("Float32T Final BitVec: {}", bitvec));
        let decoded = get_float32t(&bitvec, 16);
        println!("Decoded Value: {}", decoded);
        assert_eq!(decoded, -281.25);
    }

    #[test]    
    fn test_get_uintegert4() {
        let mut bitvec: BitVec = std::iter::repeat(false).take(32).collect();
        encode_uintegert(&mut bitvec, 2, 12, "254");
      
        println!("Raw Vector: {:?}", bitvec);
        let x = get_uintegert(&bitvec, 2, 12);
        assert_eq!(x, 254);
    }
    #[test]
    fn test_get_integert1() {
        let mut bitvec: BitVec = std::iter::repeat(false).take(32).collect();
        encode_integert(&mut bitvec, 0, 32, "-254");
        let x = get_integert(&bitvec, 0, bitvec.len());
        assert_eq!(x, -254);
    }

    #[test]
    fn test_get_booleant() {
        let mut bitvec: BitVec = std::iter::repeat(false).take(32).collect();

        bitvec.store(0x11D7);
        println!("Raw Vector: {:?}", bitvec);
        let x = get_uintegert(&bitvec, 0, bitvec.len());
        assert_eq!(x, 4567);
    }
    #[test]
    fn test_get_uintegert3() {
        let mut bitvec: BitVec = std::iter::repeat(false).take(32).collect();
        bitvec.store(4545);
        println!("Raw Vector: {:?}", bitvec);
        let x = get_uintegert(&bitvec, 0, bitvec.len());
        assert_eq!(x, 4545);
    }
}
