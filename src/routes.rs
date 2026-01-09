pub struct Route {
    pub ip: String,
    pub mask: String,
    pub port: String,
}

pub fn get_routing_table() -> Vec<Route> {

    vec![  //last expression gets returned
    Route {
        ip: String::from("10.114.0.0"),
        mask: String::from("16"),
        port: String::from("1"),
    },
    Route {
        ip: String::from("10.114.128.0"),
        mask: String::from("24"),
        port: String::from("2"),
    },
    Route {
        ip: String::from("10.114.128.0"),
        mask: String::from("25"),
        port: String::from("3"),
    },
    Route {
        ip: String::from("10.114.128.32"),
        mask: String::from("27"),
        port: String::from("4"),
    },
    Route {
        ip: String::from("192.168.1.0"),
        mask: String::from("24"),
        port: String::from("5"),
    }
    ]
}