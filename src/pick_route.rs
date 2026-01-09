use crate::subnet_calc;
use crate::ip_conversions::u32_to_str_ip;
use crate::routes::Route;


pub struct Packet {
     pub destination_ip: &str,
     pub destination_mask: &str,
}

// the <'a> lets the refrence being returned to stay valid for the lifetime of routing table
//you need it after the func name, and on the param that the return type is referencing.
//in this case, we are returning one of the route sructs in the routing_table vec
pub fn pick_route<'a>(packet: &Packet, routing_table: &'a Vec<Route>) -> Option<&'a Route> { // Option<> >this lets me return none. i think this has somhting to do with enums?


    let candidates: Vec<&Route> = routing_table.iter()
    .filter(|route| route.ip == u32_to_str_ip(subnet_calc::calc_subnet(&packet.destination_ip, &packet.destination_mask)))
    .collect();

    if candidates.is_empty() {
        return None; 
    }

    let mut max = 0;
    
    candidates.iter()
         .max_by_key(|route| route.mask.parse::<u32>().unwrap())
         .copied()
    
}