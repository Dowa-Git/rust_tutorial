// 8. collection

pub mod about_vector {
    pub fn vector_usage() {
        // vector dynamic
        let mut v: Vec<i32> = Vec::new();
        v.push(5);
        v.push(6);

        // vector with initializing
        let v_init = vec![1, 2, 3];
        
        let third: &i32 = &v_init[2];                   // if index is out of range "panic!" macro will be raise
        let third: Option<&i32> = v.get(100);        // Option<T> is counter for None
        println!("aaaa : {}", third.is_none())
    }

    pub fn vector_iterate() {
        let v = vec![100, 32, 57];
        for i in &v {
            println!("{}", i);
        }
        
        let mut v2 = vec![100, 32, 57];
        for i in &mut v2 {
            *i += 50;
        }

        for i in &mut v2 {
            println!("{}", i);
        }
    }

    enum DataNode {
        Int(i32),
        Float(f64),
        Text(String),
    }

    pub fn vector_with_enum() {
        let row = vec ![
            DataNode::Int(3),
            DataNode::Text(String::from("blue")),
            DataNode::Float(10.12),
        ];
    }
}

pub mod about_string {
    pub fn string_usage() {
        //create empty string
        let mut s = String::new();

        // make String with string literal
        let data = "initial contents";
        let s = data.to_string();

        let s = String::from("initial contents");

        // String is utf-8
    }

    pub fn string_update() {
        // append string slice 
        let mut s = String::from("foo");
        s.push_str("bar");

        // append string slice and use again
        let mut s1 = String::from("foo");
        let s2 = "bar";
        s1.push_str(&s2);
        println!("s2 is {}", s2);

        // append character
        let mut s = String::from("lo");
        s.push('l');

        // attach the words
        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");
        let s3 = s1 + &s2; // s1 will be moved so can't use anymore
    }

    pub fn string_formatting() {
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");
        let ss = String::new();
        let s_add_case = ss + &s1 + "-" + &s2 + "-" + &s3;
        let s_format_case = format!("{}-{}-{}", &s1, &s2, &s3);
    }

    pub fn string_indexing() {
        // let s1 = String::from("Hello");
        // let h = s1[0];
        // let hello = "Hello";
        // let answer = &hello[0];
        // println!("{}", answer);

        // rust don't support the string indexing cause string is vector what contained 'utf-8', not the character(byte).
        // so if try indexing it's can return the incorrect data.
        // and also grapheme case, it's not the character.  
    }

    pub fn string_slicing() {
        let hello = "Hello World";
        let s = &hello[0..1];

        println!("{}", s);
    }

    pub fn string_iteration() {
        let test_word = "Hello, World!";

        for character in test_word.chars(){
            println!("{}", character);
        }
    }
}

pub mod about_hash_map {
    pub fn hash_map_usage(){
        // hashmap need import the module
        use std::collections::HashMap;
        // Create empty hash map
        let mut scores = HashMap::new();
        // insert the value to hash map
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        // Create the hash map from two vector(zip() -> tuple -> collect = hashmap) 
        let teams = vec![String::from("Blue"), String::from("Yellow")];
        let initial_scores = vec![10, 50];
        let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

        let field_name = String::from("Favorite Color");
        let field_value = String::from("Black");

        let mut map = HashMap::new();
        map.insert(field_name, field_value);
        // println!("{}", field_name); <- this line will make the compile error.
        // in this case 'field_name', 'field_value' variables moved in map Hashmap.
        
        // access to value in hashmap with key 
        let input_name = String::from("Favorite Color");
        let fav_color = map.get(&input_name);
        println!("fav : {}", fav_color.unwrap());
        
        // iterating hashmap and get key, values
        for (key, value) in &scores{
            println!("{} : {}", key, value);
        }

        // update the hashmap
        map.insert(String::from("Favorite Color"), String::from("TTT"));

        for (key, value) in &map{
            println!("{} : {}", key, value);
        }

        map.insert(String::from("Favorite Color"), String::from("AAA"));

        for (key, value) in &map{
            println!("{} : {}", key, value);
        }

        // insert the value when key is not exists
        let mut color_table = HashMap::new();
        color_table.insert(String::from("Black"), (0, 0, 0));
        color_table.insert(String::from("White"), (255,255,255));

        color_table.entry(String::from("Red")).or_insert((255,0,0));
        color_table.entry(String::from("Black")).or_insert((1, 1, 1));

        println!("{:?}", color_table);

        // update the value based on previous value
        let text = "Hello world wonderful world";
        let mut word_map = HashMap::new();
        for word in text.split_whitespace(){
            let count = word_map.entry(word).or_insert(0);
            *count += 1;
        }
        println!("{:?}", word_map);

    }
}

pub mod pracitce {
    use std::io;
    use std::{io::Write, 
        collections::HashMap
    };

    pub fn practice_1(input_list: &[i32]){
        let mut numbers =  Vec::from(input_list);
        println!("raw data = {:?}", numbers);
        numbers.sort();
        println!("sorted data = {:?}", numbers);

        use std::collections::HashMap;
        let mut count_map = HashMap::new();
        let mut sum = 0;

        for num in &numbers{
            sum += num;
            let count = count_map.entry(num).or_insert(0);
            *count += 1;
        }

        let mut mode:Option<(&i32, &i32)> = None;
        for (number, counter) in &count_map{
            if mode.is_none(){
                mode = Some((number, counter));
            }
            else {
                if mode.unwrap().1 < counter{
                    mode = Some((number, counter));
                }
            }
        }

        let average: f64 = sum as f64 / numbers.len() as f64;

        // println!("count_map = {:?}", count_map);
        println!("total = {}", sum);
        println!("average = {}", average);
        println!("median = {}", numbers[numbers.len() / 2]);
        println!("mode = {}", mode.unwrap().0);
    }

    pub fn practice_2 (mut input_string :String){
        let vowels = vec!['a', 'e', 'i', 'o', 'u'];
        let vowel_string = String::from("aeiou");

        let first_character = input_string.get(0..1);

        let vowel_seeker_from_str = vowel_string.find(first_character.unwrap());
        let vowel_seeker_from_vec = vowels.iter().find(|v| v.to_string() == first_character.unwrap());

        let mut result = String::new();
        if vowel_seeker_from_str.is_none(){
            result.push_str(input_string.get(1..input_string.len()).unwrap());
            result.push('-');
            result.push_str(first_character.unwrap());
            result.push_str("ay");
        }
        else{
            result.push_str(&input_string);
            result.push_str("-hay");
        }
        println!("{}", result);
    }


    pub fn practice_3(){
        let mut deploy  = HashMap::new();
        loop {
            print!(">> ");
            io::stdout().flush().expect("HH");
            let mut command = String::new();

            io::stdin().read_line(&mut command)
                .expect("Failed to read line");

            let split_command :Vec<&str> = command.trim().split_whitespace().collect();

            if split_command[0] == "Add"{
                let target = deploy.entry(String::from(split_command[1])).or_insert(Vec::new());
                target.push(split_command[2]);
            }
            else if split_command[0] == "List"{
                if split_command[1] == "All"{
                    
                }
                else if split_command[1] == "Sort"{

                }
            }
        }
    }
}