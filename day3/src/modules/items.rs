pub fn find_duplicate_on_second_half( p: &[u8] ) -> Option<&u8> {
    
    let mid = p.len() >> 1;
    let mut filter = 0u64; // 54 possible characters

    for i in &p[..mid] {
        filter |= 1 << ( i % 64 ); 
    }

    for i in &p[mid+1..] {

        if filter & 1 << ( i % 64  ) != 0 {
            return Some( i )
        }    
    } 
    
    None
}