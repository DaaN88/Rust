fn main() {
    let lines_of_other_couplets: [&str; 11] = [
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five goldenen rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten Lords a Leaping",
        "Eleven Pipers Piping",
        "12 Drummers Drumming",
    ];

    let mut last_couplet_line: &str = "a Partridge in a Pear Tree";

    // here 0 is the first day
    let mut day_in_order: usize = 0;

    while day_in_order < 12 {
        println!(
            "On the {} day of Christmas\nMy true love brought to me",
            get_day_text(day_in_order),
        );

        for day in 0..(day_in_order) {
            println!("{}", lines_of_other_couplets[day]);
        }

        if day_in_order > 0 {
            last_couplet_line = "And a Partridge in a Pear Tree";
        }

        println!("{}\n", last_couplet_line);

        day_in_order += 1;
    }
}

//==================================================================================================

fn get_day_text(day_as_number: usize) -> &'static str {
    return match day_as_number {
        0 => "first",
        1 => "second",
        2 => "third",
        3 => "fourth",
        4 => "fifth",
        5 => "sixth",
        6 => "seventh",
        7 => "eighth",
        8 => "ninth",
        9 => "tenth",
        10 => "eleventh",
        11 => "twelfth",
        _ => panic!("day not supported."),
    }
}
