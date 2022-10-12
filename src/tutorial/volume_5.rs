
pub mod structure_basic_syntax{
    struct User {
        username: String,
        email: String,
        sing_in_count: u64,
        active: bool,
    }

    pub fn structure_test(){
        let mut user1 = User {
            email: String::from("someone@example.com"),
            username: String::from("someusername123"),
            active: true,
            sing_in_count: 1,
        };
        // simple structure initialize
        user1.email = String::from("anotheremail@example.com");
        println!("user1.email is {}", user1.email);
    
        // structure initialize with function
        let mut user2 = build_user(String::from("aaa@test.com"), String::from("TestName"));
        println!("user2.name is {} // user2.email is {}",user2.username, user2.email);
        println!("Change user2.name to BBB");
        user2.username = String::from("BBB");
        println!("user2.name is {}", user2.username);
    
        // structure initialize from another instance
        let user3 = User {
            email: String::from("another@example.com"),
            username: String::from("anotherusername567"),
            active: user1.active,
            sing_in_count: user1.sing_in_count,
        };
    
        // structure initialize from another instance with struct update syntax
        let user4 = User {
            email: String::from("another1@example.com"),
            username: String::from("anotherusername890"),
            ..user1
        };
    
        // Structure with no filed name
        struct Color(i32, i32, i32);
        struct Point(i32, i32, i32);
        let black = Color(0, 0, 0);
        let origin = Point(0, 0, 0);
    
    
    
    }
    
    fn build_user(email: String, username: String) -> User {
        User {
            // if parameter name is same with field name, it can be shorthand.
            email,           // email: email,
            username,        // username: username
            active: true,
            sing_in_count: 1,
        }
    }
}

pub mod structure_example{
    // example 1
    pub fn structure_example() {
        let length1 = 50;
        let width1 = 30;
        
        println!(
            "The area of the rectangle is {} square pixels.",
            area(length1, width1)
        );
    }

    fn area(length: u32, width: u32) -> u32 {
        length * width
    }

    // example 2 : refactoring with tuple
    pub fn structure_example_enhance() {
        let rect = (50, 30);
        println!(
            "The area of the rectangle is {} square pixels.",
            area2(rect)
        );
    }

    fn area2(dimensions: (u32, u32)) -> u32 {
        dimensions.0 * dimensions.1
    }

    // example 3 : refactoring with structure
    pub fn structure_example_enhance2() {
        let rect = Rectangle{length: 50, width: 30};
        println!(
            "The area of the rectangle is {} square pixels.",
            area3(&rect)
        )
    }

    struct Rectangle {
        length: u32,
        width: u32,
    }

    fn area3(rectangle: &Rectangle) -> u32 {
        rectangle.length * rectangle.width
    }

    #[derive(Debug)]
    struct RectangleTrait {
        length: u32,
        width: u32,
    }

    pub fn display_debug_rect() {
        let rect1 = RectangleTrait { length: 50, width: 30 };
        println!(":? rect1 is {:?}", rect1);
        println!(":#? rect1 is {:#?}", rect1);
    }
}

pub mod structure_method{
    #[derive(Debug)]
    struct Rectangle {
        length: u32,
        width: u32,
    }
    impl Rectangle{
        // method
        fn area(&self) -> u32 {
            self.length * self.width
        }
        
        fn can_hold(&self, other:&Rectangle) -> bool {
            self.area() > other.area()
        }

        // associate function
        fn square(size: u32) -> Rectangle {
            Rectangle { length: size, width: size }
        }
    }

    pub fn method_test(){
        let rect1 = Rectangle { length: 50, width: 30};

        println!(
            "The area of the rect1 is {} square pixels",
            rect1.area()
        );

        let rect2 = Rectangle { length: 40, width: 10 };
        let rect3 = Rectangle::square(50);

        println!("Can rect1 hold rect2? : {}", rect1.can_hold(&rect2));
        println!("Can rect1 hold rect3? : {}", rect1.can_hold(&rect3));
    }
}