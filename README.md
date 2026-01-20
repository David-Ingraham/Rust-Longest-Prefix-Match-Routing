# Longest Prefix Matching
Heavily commented Rust module for performing longest prefix matching on IPv4addresses. 

Code is tested against a hardcoded vector of route structs containing a destination IP, interface, and subnet

```rust
pub struct Route {
    pub ip: String,
    pub mask: String,
    pub port: String,
}
```

Hardcoded incoming packet data structure 

```rust
pub struct Packet <'a> { 
                       
     pub destination_ip: &'a str,
     pub destination_mask: &'a str,
}
```

pick_route.rs just returns the route stuct for the network that matches the subnet of the destination ip and has the longest mask

```rust

 let candidates: Vec<&Route> = routing_table.iter()
    .filter(|route| route.ip == u32_to_str_ip(subnet_calc::calc_subnet(&packet.destination_ip, &packet.destination_mask)))
    .collect();

candidates.iter()
    .max_by_key(|route| route.mask.parse::<u32>().unwrap())
    .copied()
```

subnet calc calls the ip conversions and generates a run of 1s for the mask length, and 0s for the host bits to get a u32 subnet id

```rust
let u32_mask = !((1 << (32 - prefix_len)) -1);
ip_u32 & u32_mask
```

conversions is the fun part. i assume ip addresses will come as a string forwhen calling the route table from an API or CLI output, 

## IP Conversions

String to u32. Split on dot delimiter. Map each octet to u8. Shift bits left enough times to place the u8 far enough into the u32 so ifreflects its octect and OR together. 

```rust
let ip_u32 = (a as u32) << 24 
| (b as u32) << 16 
| (c as u32) << 8  
| (d as u32);
```

u32 to string. Shift bits right. Mask with 0xFF to isolate octets.

```rust
format!("{}.{}.{}.{}", 
    (ip_u32 >> 24) & 0xFF,
    (ip_u32 >> 16) & 0xFF,
    (ip_u32 >> 8) & 0xFF,
    ip_u32 & 0xFF)
```

## Project Modularization

lib.rs exposes modules as public for external access.

```rust
pub mod pick_route;
pub mod subnet_calc;
pub mod ip_conversions;
pub mod routes;
```

Cargo.toml defines package metadata. Edition 2024. No dependencies.

Cargo.lock automatically generated. Locks dependency versions for reproducible builds.

  
