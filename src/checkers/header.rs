use crate::constants;

pub fn check_magic_header(magic_bytes:&[u8])->bool{
    let mut is_valid = true;
    for i in 0..16{
        if constants::ELF_MAGIC_BYTES[i]!=magic_bytes[i]{
            is_valid = false;
            break;
        }
    }
    return is_valid;
}