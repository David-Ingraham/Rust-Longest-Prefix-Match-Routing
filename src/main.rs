mod subnet_calc;
mod ip_conversions;
mod routes;
mod pick_route;



fn main() {
    
    let destination_ip = "192.168.1.1";
    let destination_mask = "11"; // string literal, already type &str, or string refrence

    let mask_vec: Vec<u32> = (1..32).collect(); //vec![0..32];// 0..32 return a range object or struct
    //let mask_array: <&str> = mask_u32_vec.map(|&x| x.to_string()).collect();


    let str_subnet = ip_conversions::u32_to_str_ip(subnet_calc::calc_subnet(destination_ip, destination_mask));//would be nice if this func had a flag param to chnage return type 

    //let str_subnet = u32_to_str_ip(u32_subnet);

    println!("Subnet: {}", str_subnet);

    for mask in mask_vec{
        let str_mask: String  = mask.to_string(); // returns String but type coercion when passing it to calc_subnet func
        //does not need to be mutable. str_mask goes out of scope after each iteration
        
        let strsubnet = ip_conversions::u32_to_str_ip(subnet_calc::calc_subnet(destination_ip, &str_mask));
        println!("Ip: {}, Subnet: {}, Mask: {}", destination_ip, str_subnet, str_mask);
    }



    let routing_table = routes::get_routing_table();
    let packet = pick_route::Packet {
        destination_ip,
        destination_mask
    };

    let best_route = pick_route::pick_route(&packet, &routing_table);

    match best_route {
        Some(route) => println!("Packet intended for: {}, getting routed out port: {}", packet.destination_ip, best_route.port),
        None => println!("No subnet route found to match destination")
    }
}
