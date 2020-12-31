fn main() {
    println!("The Lyrics to 'The Twelve Days of Christmas'.");

    for i in 1..13 {
    	let day = match i {
    		1 => "first",
    		2 => "second",
    		3 => "third",
    		4 => "fourth",
    		5 => "fifth",
    		6 => "sixth",
    		7 => "seventh",
    		8 => "eighth",
    		9 => "nineth",
    		10 => "tenth",
    		11 => "eleventh",
    		12 => "twelveth",
    		_  => "Oops",
    	};
    	println!("");
    	println!("On the {} of Christmas", day);
    	println!("my true love sent to me:");

    	if i == 1 {
    		println!("A Partridge in a Pear Tree");
    	} else {
    		twelve_days(i);
    	};
    };
}

fn twelve_days(i: u64) {
	match i {
    	1 => println!("and A Partridge in a Pear Tree"),
    	2 => {
    			println!("Two Turtle Doves");
    			twelve_days(i - 1);
    	}
    	3 => {
    			println!("Three French Hens");
    			twelve_days(i - 1);
    	}
    	4 => {
    			println!("Four Calling Birds");
    			twelve_days(i - 1);
    	}
    	5 => {
    			println!("Five Golden Rings");
    			twelve_days(i - 1);
    	}
    	6 => {
    			println!("Six Geese a Laying");
    			twelve_days(i - 1);
    	}
    	7 => {
    			println!("Seven Swans a Swimming");
    			twelve_days(i - 1);
    	}
    	8 => {
    			println!("Eight Maids a Milking");
    			twelve_days(i - 1);
    	}
    	9 => {
    			println!("Nine Ladies Dancing");
    			twelve_days(i - 1);
    	}
    	10 => {
    			println!("Ten Lords a Leaping");
    			twelve_days(i - 1);
    	}
    	11 => {
    			println!("Eleven Pipers Piping");
    			twelve_days(i - 1);
    	}
    	12 => {
    			println!("12 Drummers Drumming");
    			twelve_days(i - 1);
    	}
    	_ => println!("Oops"),
    };
}