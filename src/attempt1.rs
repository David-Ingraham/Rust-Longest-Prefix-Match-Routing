struct Route {
    network: String,
    network_binary: Vec<u8>,
    mask: u8,
    port: u32,




}

fn get_network_binary(ip: &str) -> Vec<u8>{

    let mut octets: Vec<&str> = ip.split(".").collect();

    for octet in octets{
        let octect: u8 = octet.parse().unwrap(); // somehow parsing but not saving? i need to read that varibale section of the book again.
        //is this rust being coy about copy operations? conservative with it?  
    }

    return octets;
}   

fn calculate_subnet_id(route: &Route, mask: u8) -> Vec<u8>{

    let hostbits = 32 - mask;

    let mut single_bin_rep_mask = String::from("");

    for octet in route.network_binary{
        single_bin_rep_mask += &octet.to_string();
    }

    for i in route.mask..32{
        single_bin_rep_mask[i as usize] = "0"; //trying to set hostbits to 0 which would reveal the subnet id
    }    //but this does not work bc strings are not indexible or mutable, even tho i declared single_bin_rep_mask as mutable?

    for i in 0..8..route.mask{  //wanted (0..route.mark).step_by(8)
        single_bin_rep_mask[i as usize] = single_bin_rep_mask[i as usize].insert(".");
    }

    let subnet_id Vec<u8> = single_bin_rep_mask.split(".").collect();

    for octect in subnet_id{
        octect: u8 = octect.parse().unwrap();
    }

    return subnet_id;
}

//fn match_longest_prefix(route: &Route, routing_table:&Vec<Route>) -> &Route{


  //  for route in routing_table{

  //  }
//}
fn main() {
    let route1 = Route {
        network: String::from("10.114.0.0"),
        network_binary: get_network_binary("10.114.0.0"),
        mask: 16,
        port: 1,
    };
    
    let route2 = Route {
        network: String::from("10.114.128.0"),
        network_binary: get_network_binary("10.114.128.0"),
        mask: 24,
        port: 2,
    };
    
    let route3 = Route {
        network: String::from("10.114.128.0"),
        network_binary: get_network_binary("10.114.128.0"),
        mask: 25,
        port: 3,
    };
    
    let route4 = Route {
        network: String::from("10.114.128.32"),
        network_binary: get_network_binary("10.114.128.32"),
        mask: 27,
        port: 4,
    };
    
    let route5 = Route {
        network: String::from("192.168.1.0"),
        network_binary: get_network_binary("192.168.1.0"),
        mask: 24,
        port: 5,
    };
    
    let route6 = Route {
        network: String::from("172.16.0.0"),
        network_binary: get_network_binary("172.16.0.0"),
        mask: 12,
        port: 6,
    };
    
    let route7 = Route {
        network: String::from("10.0.0.0"),
        network_binary: get_network_binary("10.0.0.0"),
        mask: 8,
        port: 7,
    };
    
    let route8 = Route {
        network: String::from("0.0.0.0"),
        network_binary: get_network_binary("0.0.0.0"),
        mask: 0,
        port: 8,
    };
    
    let routing_table = vec![route1, route2, route3, route4, route5, route6, route7, route8];
    
    println!("Routing table has {} routes", routing_table.len());


    println!("Subnet ID for route1: {:?}", calculate_subnet_id(&route1));

}