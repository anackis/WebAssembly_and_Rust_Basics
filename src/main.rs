

// --------------------------------------------------------------------------------//
// fn add(num_one: i32, num_two: i32) -> i32 {
//     num_one + num_two
// }

// fn main() {             // fn same as function in JS
//     // let mut my_name = "Alex";   // We need to add "mut" if we want to change variable. 
//     // my_name = "John";

//     // let foo: bool;

//     // let _foo = add(10, 5);

//     // Difference between macros and Functions:
//     // ___________________________________
//     // Macros support the following:     |   
//         // Variadic arguments            |      
//         // Pretty Syntax                 | 
//         // Metaprogramming               |                                
//     // __________________________________|
//     // "!" means that it is macros. 

//     // println!("{}", _foo);            // -> 15
//     // println!("{} {}", _foo, true);   // -> 15 true 
//     // println!("{0} {0}", _foo);       // -> 15 15
//     // println!("{:?}", _foo);             // -> 15
// }
// --------------------------------------------------------------------------------//



// // --------------------------------------------------------------------------------//
// fn add(num_one: i32, num_two: i32) -> i32 {
//     num_one + num_two
// }

// fn main() {            
//     let total = add(10, 5);

//     if total > 50 {
//         println!("You qualify for free shipping!");  
//     }
//     else if total > 20 {
//         println!("If you add more items, you can qualify for free shipping!");  
//     }
//     else {
//         println!("No free shipping!");  
//     }
    
//     println!("{:?}", total);  
// }
// // --------------------------------------------------------------------------------//




// // --------------------------------------------------------------------------------//
// fn add(num_one: i32, num_two: i32) -> i32 {
//     num_one + num_two
// }

// fn main() {            
//     let mut total = add(10, 5);
//     let mut free_shipping = false;

//     if total > 50 {
//         println!("You qualify for free shipping!");  
//         free_shipping = true;
//     }
//     else if total > 20 {
//         println!("If you add more items, you can qualify for free shipping!");  
//     }
//     else {
//         println!("No free shipping!");  
//     }

//     // match free_shipping {
//     //     true => total = total + 0,
//     //     false => total = total + 5
//     // }

//     total = match free_shipping {
//         true => total + 0,
//         false => total + 5
//     };

//     match total {
//         1 => println!("1"),
//         2 => println!("2"),
//         _ => println!("No match found!")        // "_" for case that are not matched. 
//     }
    
//     println!("Total: {:?}", total);  
// }
// // --------------------------------------------------------------------------------//



// // --------------------------------------------------------------------------------//
// fn add(num_one: i32, num_two: i32) -> i32 {
//     num_one + num_two
// }

// fn main() {            
//     let mut total = add(10, 5);
//     let mut free_shipping = false;

//     if total > 50 {
//         println!("You qualify for free shipping!");  
//         free_shipping = true;
//     }
//     else if total > 20 {
//         println!("If you add more items, you can qualify for free shipping!");  
//     }
//     else {
//         println!("No free shipping!");  
//     }

//     total = match free_shipping {
//         true => total + 0,
//         false => total + 5
//     };

//     match total {
//         1 => println!("1"),
//         2 => println!("2"),
//         _ => println!("No match found!")        // "_" for case that are not matched. 
//     }
    
//     // println!("Total: {:?}", total);  

//     let items = [1, 2, 3, 4, 5];
//     println!("{:?}", items); 

// }
// // --------------------------------------------------------------------------------//




// --------------------------------------------------------------------------------//
// _________________________________|
// Vectors                          |
// _________________________________|
// Arrays are fixed-size.           |
// Vectors are dynamically-sized.   |
// _________________________________|

// fn main() {            

//     let items = [1, 2, 3, 4, 5];                                    // Array for fixed amount of items inside of array
//     println!("{:?}", items); 

//     let vector_items = vec![1, 2, 3, 4, 5];                         // Vectors for unknown amount of items inside of the vectors

//     let mut vector_items_2 = Vec::new();
//     vector_items_2.push(1);
//     vector_items_2.push(2);
//     vector_items_2.push(3);
//     vector_items_2.push(4);
//     vector_items_2.push(5);

//     println!("{:?}", vector_items);  
//     println!("{:?}", vector_items_2);  

// }
// --------------------------------------------------------------------------------//




// // --------------------------------------------------------------------------------//
// // Structures //
// //-----------//

// struct BankAccount {
//     balance: i32,
//     verified: bool
// }

// fn main() {             // fn same as function in JS
//     let my_account = BankAccount {
//         balance : 20,
//         verified: true
//     };

//     println!("{:?}", my_account.balance);
//     println!("{:?}", my_account.verified);
// }
// // --------------------------------------------------------------------------------//




// --------------------------------------------------------------------------------//
// Ownership //
//-----------//

struct BankAccount {
    balance: i32,
    verified: bool
}

fn print_balance(account: &BankAccount) {           // & = borrow
    println!("{:?}", account.balance);
}

fn print_verified(account: &BankAccount) {          // & = borrow
    println!("{:?}", account.verified);
}

fn main() {             // fn same as function in JS
    let my_account = BankAccount {
        balance : 20,
        verified: true
    };

    print_balance(&my_account);         // & = borrow
    print_verified(&my_account);         // Error. my_account has been removed.   // & = borrow helps us with error without "&"
}
// --------------------------------------------------------------------------------//

