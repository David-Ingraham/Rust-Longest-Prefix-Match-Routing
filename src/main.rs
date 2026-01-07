mod subnet_calc;

fn main() {
    
    let ip = "192.168.1.1";
    let mask = "24";
    let u32_subnet = subnet_calc::calc_subnet(ip, mask);

    let subnet = format!("{}.{}.{}.{}", 
    (u32_subnet >> 24) & 0xFF, // taking first 1/4th of the bits from the u32 subnet, shifting that first bit in (starting at 8th position)
                               //to the right 24 places so it ends up 32 from the right, or at the begining of the u32. 
                               // the & OxFF turns the remaining 24 bits in to 0. Ox means hexadecimal, FF is 1^16 + 2^16 = 256 or 11111111 in binary
                               // & (and) bit operation against all zeros turns operand into all zeros 
    (u32_subnet >> 16) & 0xFF, // same process but starting 16 bits from the right to not undo the work in the first octet
    (u32_subnet >> 8) & 0xFF,
    u32_subnet & 0xFF);  // no bitshift needed since the first bit of this section is aldready at bit 8 in the u32. 
                        // just need to zero out the bits in fornt of it

    //


    println!("Subnet: {}", subnet);
}
