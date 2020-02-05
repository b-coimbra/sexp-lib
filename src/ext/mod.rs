pub fn to_u32(str: &String) -> u32 {
    str.trim().parse().expect("Not a number!")
}
