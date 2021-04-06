fn main() {
    let days = ["first", "second", "third", "fourth", "fifth", "sixth", 
        "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    let gifts = ["a partridge in a pear tree",
                 "two turtle doves",
                 "three French hens",
                 "four calling birds",
                 "five golden rings",
                 "six geese a-laying",
                 "seven swans a-swimming",
                 "eight maids a-milking",
                 "nine ladies dancing",
                 "ten lords a-leaping",
                 "eleven pipers piping",
                 "twelve drummers drumming"];

    for day in days.iter() {
        println!("On the {} day of Christmas my true love gave to me", day);
        let count = days.iter().position(|x| x == day);
        match count {
            Some(x) => {
                for i in (0..x).rev() {
                    println!("{}, {}", i, gifts[i]);
                }
            },
            None    => println!("Invalid day of Christmas!"),
        }
    }
}
