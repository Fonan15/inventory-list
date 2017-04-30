use std::io;

fn main() {
    let mut count = 0;
    loop{
//    	let in = read_var('Gegenstand');
	println!("Please type an item:");
	let mut in = String::new();
	
	io::stdin().readline(&mut in)
		.expect("Failed to read line");
//Hier fängt der alte Teil wieder an
        println!("Die Eingabe war: {}",in);
	putinlist(in);
	println!("Die Liste sieht nun wiefolgt aus: {}", list);
	count = count+1;
    }
}

fn read_var (c:String){
    println!("Please type {}", c); 
    let mut v = String::new();

    io::stdin().read_line(&mut v)
    	.expect("Failed to read line!");

}

fn putinlist(in: String) -> [&str; 80]{

}
