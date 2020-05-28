//! Simple inventory management program, based on the "Student's records" program 
//! from the FastBitLabs "Embedded C for absolute beginners" course on Udemy
//! with some modifications and additions.
//! 
//! TO DO: correctly handle input errors (e.g. user types a text instead of an integer for ID)


use std::io;

struct Record {
    name: String,
    id: u16,
    price: f32,
    quantity: u16,
}

fn main() {

    let mut inventory: Vec<Record> = Vec::new();

    let mut continue_program: bool = true;

    let error_msg: &str = "Failed to read input";

    println!("Inventory management program");

    while continue_program {

        display_menu();

        let mut option = String::new();
        io::stdin().read_line(&mut option).expect(&error_msg);
        let option: u8 = option.trim().parse().expect(&error_msg);

        if option == 1 {
            display_items(&inventory);
        } else if option == 2 {
            add_item(&mut inventory);
        } else if option == 3 {
            delete_item(&mut inventory);
        } else if option == 4 {
            display_stats(&inventory);
        } else if option == 0 {
            continue_program = false;
        } else {
            println!("invalid input");
        }

    }

}

fn display_menu() {
    println!("\nDisplay inventory     ->1");
    println!("Add an item           ->2");
    println!("Delete an item        ->3");
    println!("Inventory statistics  ->4");
    println!("Exit program          ->0");
}


fn display_items(records: &Vec<Record>) {

    //display all the existing records

    let size: usize = records.len();

    println!("There are {} records in the database.", size);

    for record in records.iter() {
        let value: f32 = record.price * record.quantity as f32;

        println!("ID: {}\tname: {}\tprice: {:0.2}€\tquantity: {}\tvalue: {:0.2}€",
                record.id, record.name, record.price, record.quantity, value);

    }

}

fn display_stats(records: &Vec<Record>){

    //display some info about the inventory
    let size: usize = records.len();
    let mut total_value: f32 = 0.0;
    let mut total_quantity: u16 = 0;
    let mut min_price: f32 = 0.0;
    let mut max_price: f32 = 0.0;

    if size > 0 {
        min_price = records[0].price;
        max_price = records[0].price;
    }
    

    for record in records.iter() {
        let value: f32 = record.price * record.quantity as f32;
        total_value +=  value;
        total_quantity += record.quantity;
        if record.price < min_price {
            min_price = record.price;
        }
        if record.price > max_price {
            max_price = record.price;
        }
    }

    println!("\nTotal value of the inventory is {:0.2}€.", total_value);
    println!("Total quantity of items in the inventory is {}.", total_quantity);
    
    if size > 0 {
    println!("Price range is {:0.2}€ to {:0.2}€.", min_price, max_price);
}



}

fn add_item(records: &mut Vec<Record>) {
    
    //add an item
    //check if there are any empty slots left
    //check if there is an item with the same id already

    let size: usize = records.len();

    if size > 9 {
        println!("No more empty slots!");
    } else {

        println!("Insert the ID: ");
        let mut new_id = String::new();
        io::stdin().read_line(&mut new_id).expect("Failed to read input!");
        let new_id: u16 = new_id.trim().parse().expect("Failed to read input!");

        if size == 0 {
            let (new_name, new_price, new_quantity) = get_info();
            let new_record: Record = Record {name: new_name.to_string(), id: new_id, price: new_price, quantity: new_quantity};
            records.push(new_record);
            
        } else {
            for idx in 0..size {
                if records[idx].id == new_id {
                    println!("Error: ID already present in the database!"); 
                    break;
                } else {
                    let (new_name, new_price, new_quantity) = get_info();
                    let new_record: Record = Record {name: new_name.to_string(), id: new_id, price: new_price, quantity: new_quantity};
                    records.push(new_record);
                    
                }
           }

        }

    
    }

}

fn get_info() ->(String, f32, u16) {

    println!("Insert product name: ");
    let mut new_name = String::new();
    io::stdin().read_line(&mut new_name).expect("Failed to read input!");
    let new_name: String = new_name.trim().parse().expect("Failed to read input!");
                
    println!("Insert product price: ");
    let mut new_price = String::new();
    io::stdin().read_line(&mut new_price).expect("Failed to read input!");
    let new_price: f32 = new_price.trim().parse().expect("Failed to read input!");
        
    println!("Insert product quantity: ");
    let mut new_quantity = String::new();
    io::stdin().read_line(&mut new_quantity).expect("Failed to read input!");
    let new_quantity: u16 = new_quantity.trim().parse().expect("Failed to read input!");

    return (new_name, new_price, new_quantity);

}


fn delete_item(records: &mut Vec<Record>) {
    
    let size: usize = records.len();    

    println!("Insert the ID to delete: ");
    let mut new_id = String::new();
    io::stdin().read_line(&mut new_id).expect("Failed to read input!");
    let new_id: u16 = new_id.trim().parse().expect("Failed to read input!");

    for idx in 0..size {
        if records[idx].id == new_id {
            records.remove(idx);
            break;
        } else {
            println!("Error: no such ID in the database!");
            break;
        }
    }
}



