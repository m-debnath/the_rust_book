fn main() {
    for i in 1..13 {
        println!("On the {} day of Christmas", numeric_presentation(i));
        println!("My true love sent to me");
        let mut n = i;
        while n > 0 {
            println!("{}", lyric_presentation(n, if i == 1 { false } else { true }));
            n -= 1;
        }
        println!("");
    }
}

fn lyric_presentation(n: u32, more_than_one: bool) -> String {
    if n == 1 {
        if more_than_one {
            "And a partridge in a pear tree".to_string()
        } else {
            "A partridge in a pear tree".to_string()
        }
    } else if n == 2 {
        "Two turtle-doves".to_string()
    } else if n == 3 {
        "Three French hens".to_string()
    } else if n == 4 {
        "Four calling birds".to_string()
    } else if n == 5 {
        "Five golden rings".to_string()
    } else if n == 6 {
        "Six geese a-laying".to_string()
    } else if n == 7 {
        "Seven swans a-swimming".to_string()
    } else if n == 8 {
        "Eight maids a-milking".to_string()
    } else if n == 9 {
        "Nine ladies dancing".to_string()
    } else if n == 10 {
        "Ten lords a-leaping".to_string()
    } else if n == 11 {
        "Eleven pipers piping".to_string()
    } else if n == 12 {
        "Twelve drummers drumming".to_string()
    } else {
        "".to_string()
    }
}

fn numeric_presentation(n: u32) -> String {
    if n == 1 {
        "first".to_string()
    } else if n == 2 {
        "second".to_string()
    } else if n == 3 {
        "third".to_string()
    } else if n == 4 {
        "fourth".to_string()
    } else if n == 5 {
        "fifth".to_string()
    } else if n == 6 {
        "sixth".to_string()
    } else if n == 7 {
        "seventh".to_string()
    } else if n == 8 {
        "eighth".to_string()
    } else if n == 9 {
        "ninth".to_string()
    } else if n == 10 {
        "tenth".to_string()
    } else if n == 11 {
        "eleventh".to_string()
    } else if n == 12 {
        "twelfth".to_string()
    } else {
        "".to_string()
    }
}
