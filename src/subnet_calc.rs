pub fn calc_subnet(ip: &str, mask: u8)-> u32 {

  //  let mut octets: Vec<&str> = ip.split(".").collect();
     // ** wasting space by creating the octect vecotr ****
    //let mut u8_octect_vec: Vec<u8> = vec![];

    //for octet in octets{
    //    u8_octect_vec.push(octet.parse().unwrap());
    //}

    let u8_octect_vec: Vec<u8> = ip.split(".")  // creating one vector from the ip string
        .map(|octet| octet.parse().unwrap())
        .collect(); // could use .collect()::<Vec<u8>>() turbofish but i specify the type in vector declaration / asignment

    let a = u8_octect_vec[0];
    let b = u8_octect_vec[1];
    let c = u8_octect_vec[2];
    let d = u8_octect_vec[3];

    let ip_u32 = (a as u32) << 24 
    | (b as u32) << 16 
    | (c as u32) << 8  
    | (d as u32);
    //since each letter is a u8, think about pushing a 8 bit number onto a u32 from the right
    // (a <<24 ) the first bit of a would be at the 8th bit from the right of the u32, so shifting it 24 bits places 
    //the first bit of a is no at 24 + 8 = 32nd bit position in the u32. so on with each octet
    //each expression containerd in the () is a u32, so we use the | which is the or operator. 
    // this is inclusive or, so if either it is 1 the results is 1. when doing bit operations on multi bit numbers, it goes bit by bit
    //11000000_00000000_00000000_00000000  (a as u32 << 24)
    //| 00000000_10101000_00000000_00000000  (b as u32 << 16)
    //| 00000000_00000000_00000001_00000000  (c as u32 << 8)
    //| 00000000_00000000_00000000_00000001  (d as u32)
    //= 11000000_10101000_00000001_00000001  (192.168.1.1)
  
   let u32_mask = !((1 << (32 - mask)) -1);

   // (1 << (32 - mask)) bits shfits 1, (single bit) 32- mask bumber of places to the left 
   // subtracting 1 from that tunrs all the zeros to left into 1, and tunr the original 1 into a zero, leaving you with a run of 32 - mask lenght of zeros
   // taking the inverse with ! of that so that we get a run of 1 of mask lenght follow by a run of 32 - mask lenght of zeros


    //let subnet_u32 = ip_u32 & mask; //i dont have to use return statment becuase the last expression is the return value? would be an expression 
    //if i dind thave the semi colon 

    ip_u32 & u32_mask  // no semi colon, just the return value


}


