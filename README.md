Heavily commented Rust module for performing longest prefix matching on IPv4addresses. 

Code is testing a aginst a hardcoded vector of route structs containing a destination IP, interface, and subnet

hardcoded incoming packet data structure 

pick route just returns the outgoing interface by performing subnet calc on destination ip with its CIDR mask

subnet calc calls conversions and just &s the ip with the mask


conversions is the fun part. i assume ip addresses will come as a string forwhen calling the route table from an API or CLI output, 

  
