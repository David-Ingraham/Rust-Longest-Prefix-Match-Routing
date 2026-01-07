mod subnet_calc;

fn main() {
    
    let ip = "192.168.1.1";
    let mask: u8 = 24;
    let u32_subnet = subnet_calc::calc_subnet(ip, mask);

    let subnet = format!("{}.{}.{}.{}", 
    (u32_subnet >> 24) & 0xFF,
    (u32_subnet >> 16) & 0xFF,
    (u32_subnet >> 8) & 0xFF,
    u32_subnet & 0xFF);
    println!("Subnet: {}", subnet);
}
