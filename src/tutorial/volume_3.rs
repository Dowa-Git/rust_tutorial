// 3. generic programming


// 1. Variables
// 2. Data types
pub fn variable() {
    // need concern about mutable, immutable
    let mut number_x = 5;
    println!("The value of x is : {}", number_x);
    number_x = 6;
    println!("The value of x is : {}", number_x);

    // if mutable variable need change
    // use let expression again.
    // this expression called 'shadow'
    let test_number = 5;
    println!("mutable variable testNumber is : {}", test_number);
    let test_number = test_number + 1;
    println!("mutable variable testNumber is : {}", test_number);

    // Variable Type
    // float
    // float variable declared 'f64' defaultly
    let float_x = 2.0;
    println!("The value of float_x is : {}", float_x);
    let float_y: f32 = 3.0;
    println!("The value of float_y is : {}", float_y);

    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("Tuple tup is {:#?}", tup);
    // tuple can be break to variables like under below
    let (tup_x, tup_y, tup_z) = tup;
    println!("The value of tup_x is : {}", tup_x);
    println!("The value of tup_y is : {}", tup_y);
    println!("The value of tup_z is : {}", tup_z);
    // call the element of tuple with specific index number
    let tup_first = tup.0;
    let tup_second = tup.1;
    let tup_third = tup.2;
    println!("Tuple tup's first elenet is {}",tup_first);
    println!("Tuple tup's second elenet is {}",tup_second);
    println!("Tuple tup's third elenet is {}",tup_third);

    // array
    // array of rust is have static length.
    let array_a = [1, 2, 3, 4, 5];
    // the way of call of element in tuple is same with other language
    let array_a_0 = array_a[0];
    let array_a_1 = array_a[1];
    println!("array_a first is {}", array_a_0);
    println!("array_a_second is {}", array_a_1);
}

// 3. functions
pub mod functions{
    pub fn function_test() {
        println!("Hello, world!");
    
        function_normal();
        function_with_single_parameter(5);
        function_with_multiple_parameters(10, 100);
        function_with_expression();
        let result = function_with_plust_one_return(999);
        println!("function_with_return = {}", result);
    }
    
    // rust don't care about the function decleared postion.
    // the function can placed below main
    pub fn function_normal() {
        println!("function_normal function called");
    }
    
    // if the function have parameter, the type of parameter must be specified
    pub fn function_with_single_parameter(x: i32) {
        println!("function_with_single_parameter function called");
        println!("test_foo_1 first parameter is {}", x)
    }
    
    pub fn function_with_multiple_parameters(x: i32, y: i32){
        println!("function_with_multiple_parameters fuction called");
        println!("test_foo_2 first parameter is {}", x);
        println!("test_foo_2 second parameter is {}", y);
    }
    
    // function and expression
    pub fn function_with_expression(){
        println!("function_with_expression function called");
        let x = 5;
        println!("The value of x is : {}", x);
    
        let y = {
            let x = 3;
            println!("The value of x inside y is : {}", x);
            // this is return line (the line without semi-colon)
            x + 1
        };
        println!("The value of y is : {}", y);
    }
    
    pub fn function_with_plust_one_return(input_number: i32) -> i32{
        println!("function_with_return function called");
        input_number + 1
    }
}

// 4. 

// 5. flow control
pub mod flowcontrol{
    pub fn flow_control_test() {
        // if expression
        let number = 3;
        compare_with_five(number);
        let number = 7;
        compare_with_five(number);
    
        check_divisible(6);
        check_divisible(16);
        check_divisible(1);
    
        if_inside_let(true);
        if_inside_let(false);
    
        example_loop(3);
        example_while(5);
        
        let test_array = [10, 20, 30, 40, 50];
        example_array_indexing_while(&test_array);
        example_array_indexing_for(&test_array);
    }
    
    pub fn compare_with_five(input_number :i32) {
        if input_number < 5 {
            println!("number {} is less than 5 : [number < 5] condition was true", input_number);
        } else {
            println!("number {} is equal with 5 or bigger than 5 : [number < 5] condition was false", input_number);
        }
    }
    
    pub fn check_divisible(input_number :i32) {
        if input_number % 4 == 0 {
            println!("input_number is divisible by 4");
        } else if input_number % 3 == 0 {
            println!("input_number is divisible by 3");
        } else if input_number % 2 == 0 {
            println!("input_number is divisible by 2");
        } else {
            println!("input_number is not divisible by 4, 3, or 2");
        }
    }
    
    pub fn if_inside_let(condition:bool){
        // in this case the return values of each if code block, must be same type.
        let number = if condition {
            1
        } else {
            0
        };
        println!("Condition : {}, change to decimal : {}",condition, number)
    }
    
    // loop
    // this is infinity loop.
    // can be escape with ^c(interrupt) or break expression
    pub fn example_loop(max_loop_count:i32){
        println!("loop start : max_counter = {}", max_loop_count);
        let mut loop_counter = 0;
        loop {
            if loop_counter >= max_loop_count{
                break;
            }
            loop_counter = loop_counter + 1;
            println!("loop counter : {}", loop_counter);
        }
        println!("loop end");
    }
    
    // while
    pub fn example_while(max_loop_count:i32){
        println!("while loop start : max_counter = {}", max_loop_count);
        let mut loop_counter = max_loop_count;
    
        while loop_counter > 0 {
            println!("while loop counter : {}", loop_counter);
            loop_counter -= 1;
        }
        println!("while loop end");
    }
    
    //for loop
    pub fn example_array_indexing_while(input_array :&[i32]){
        let input_array_length = input_array.len();
        let mut loop_counter = 0;
        while loop_counter < input_array_length{
            println!("index : {}, value : {}", loop_counter, input_array[loop_counter]);
            loop_counter += 1;
        }
    }
    
    pub fn example_array_indexing_for(input_array :&[i32]){
        for (index, value) in input_array.iter().enumerate() {
            println!("index : {}, value : {}", index, value);
        }
    }
}