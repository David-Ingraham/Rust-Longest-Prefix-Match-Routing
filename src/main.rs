mod subnet_calc;
mod ip_conversions;

use ip_conversions::u32_to_str_ip;

fn main() {
    
    let ip = "192.168.1.1";
    let mask = "24";
    let u32_subnet = subnet_calc::calc_subnet(ip, mask);

    let str_subnet = u32_to_str_ip(u32_subnet);

    println!("Subnet: {}", str_subnet);
}
