use std::io;

fn main() {
    let mut count = 0;//Initial count declaration so that it doesn't fall back to 0
    loop{
//    	let in = read_var('Gegenstand');
///////////////////////////////Alternative input:////////////////////////////////////////////
	println!("Please type an item:");
	let mut stuff = String::new();
	
	io::stdin().read_line(&mut stuff)
		.expect("Failed to read line");
	let mut list: [&str; 20];//still false type
///////////////////////////////Output part////////////////////////////////////////////////////
	println!("Die Eingabe war: {}",stuff);
	list[count] = &stuff;

	println!("Die Liste sieht nun wiefolgt aus: {:?}", list);//print of the full array (all items)
	count = count+1;//Increase of count and with it the place in the array
    }
}

//outsourced input
fn read_var (c:String){
    println!("Please type {}", c); 
    let mut v = String::new();

    io::stdin().read_line(&mut v)
    	.expect("Failed to read line!");

}

///////////ToDo://///////////
//- find right input type
//- right output (print the full array)
//- variable if you want to input/output/both/exit
