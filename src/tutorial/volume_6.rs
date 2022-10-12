// 6. enumerations, pattern matching

pub mod enumerations {
    enum IpAddrKind {
        V4,
        V6,
    }

    struct  IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    enum IpAddrVariantEnum {
        V4(String),
        V6(String),
    }

    enum IPAddrVaraintEnum2 {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    pub fn enumeration_define() {
        let four = IpAddrKind::V4;
        let six = IpAddrKind::V6;

        let home = IpAddr {
            kind: IpAddrKind::V4,
            address: String::from("127.0.0.1"),
        };

        let loopback = IpAddr {
            kind: IpAddrKind::V6,
            address: String::from("::1"),
        };

        let home_variant_enum = IpAddrVariantEnum::V4(String::from("127.0.0.1"));
        let loopback_variant_enum = IpAddrVariantEnum::V6(String::from("::1"));

        let home_variant_enum_2 = IPAddrVaraintEnum2::V4(127, 0, 0, 1);
        let home_variant_enum_2 = IPAddrVaraintEnum2::V6(String::from("::1"));
    }
}

pub mod option {
    // Stand library Option<T> for is this variable null or not
    // enum Option<T> {
    //     Some(T),
    //     None, 
    // }
    pub fn option_test(){
        let some_number = Some(5);
        let some_string = Some("a string");

        let absent_number: Option<i32> = None;
    }
}

pub mod match_flow {
    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska
    }
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents(coin: Coin) -> u32 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            },
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}", state);
                25
            },
        }
    }

    pub fn match_flow_test(){
        let result_penny = value_in_cents(Coin::Penny);
        println!("Penny Result : {}", result_penny);
        let result_quater = value_in_cents(Coin::Quarter(UsState::Alabama));
        println!("Quater Result : {}", result_quater);
    }

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    pub fn option_match_flow_test() {
        let five = Some(5);
        let six = plus_one(five);
        let none = plus_one(None);
    }

    fn placeholder_example() {
        let some_u8_value = 0u8;
        match some_u8_value {
            1 => println!("one"),
            3 => println!("three"),
            5 => println!("five"),
            _ => (), // "_" will be matched any variant.
        }
    }
}
// match need covered code for whole variant

pub mod if_let_flow {
    pub fn if_let_example() {
        let some_u8_value = Some(0u8);
        if let Some(3) = some_u8_value {
            println!("three");
        }
        else {
            println!("Not three!");
        }
    }
}