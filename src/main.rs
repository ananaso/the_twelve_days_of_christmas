fn capitalize_first_letter(s1: &str) -> String {
    let mut v: Vec<char> = s1.chars().collect();
    v[0] = v[0].to_uppercase().nth(0).unwrap();
    let s2: String = v.into_iter().collect();
    s2
}

fn main() {
    let days = ["first", "second", "third", "fourth", "fifth", "sixth", 
        "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    let gifts = ["a partridge in a pear tree",
                 "Two turtle doves",
                 "Three French hens",
                 "Four calling birds",
                 "Five golden rings",
                 "Six geese a-laying",
                 "Seven swans a-swimming",
                 "Eight maids a-milking",
                 "Nine ladies dancing",
                 "Ten lords a-leaping",
                 "Eleven pipers piping",
                 "Twelve drummers drumming"];

    for day in days.iter() {
        println!("On the {} day of Christmas my true love gave to me", day);
        let count = days.iter().position(|x| x == day);
        match count {
            Some(x) => {
                if x == 0 {
                    println!("{}", capitalize_first_letter(gifts[x]));
                } else {
                    for i in (0..=x).rev() {
                        let gift = gifts[i];
                        if i == 0 {
                            println!("And {}", gift);
                        } else {
                            println!("{},", gifts[i]);
                        }
                    }
                }
                println!("");
            },
            None    => println!("Invalid day of Christmas!"),
        }
    }
}
