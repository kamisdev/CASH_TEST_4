extern crate rand;
extern crate math;

use rand::{prelude::*, distributions::Alphanumeric};
use std::io;
use math::round;
use std::process;
use std::convert::From;
use std::collections::HashMap;

// function for creating random number
fn generate_price() -> f64 {
    let mut rng = rand::thread_rng();
    let y: f64 = rng.gen_range(1.00..10.00);
    round::ceil(y, 2)
}

fn generate_name() -> String {
    return rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect();
}

// function for comparing input price and generated random price
fn compare(price_origin: f64, price_input: f64) -> bool {
    price_input >= price_origin
}

fn input_payment(price_origin: f64) -> (f64, usize) {

    let mut price_input: f64;
    let mut tried_cnt: usize = 0;

    loop {
        println!("Please input value for payment. Or please press 'q' to quit.");
        let mut str_input = String::new();

        // input string
        io::stdin()
            .read_line(&mut str_input)
            .expect("Cannot read line");

        // check if input 'q'
        let pp = str_input.as_bytes();
        if pp[0] == b'q' {
            println!("Do you really want to exit? y/n");

            // confirm quit
            str_input = String::from("");
            io::stdin()
                .read_line(&mut str_input)
                .expect("Cannot read line");
            
            let pp = str_input.as_bytes();
            if pp[0] == b'y' {
                process::exit(1);
            } else {
                continue;
            }
        }
        
        price_input = match str_input.trim().parse() {
            Ok(pay) => {
                if pay <= 0.0 || pay >= 1000.0 {
                    println!("Invalid input! Must be between 1.00 to 999.99");
                    continue;
                }

                // check format 000.00
                let check_pay = round::ceil(pay, 2);
                if check_pay * 100.0 != pay * 100.0 {
                    
                    println!("Invalid input format! Must be format like : 000.00");
                    continue;
                }
                tried_cnt += 1;
                pay
            },
            Err(_) => {
                println!("Invalid input");
                continue;
            }
        };

        break;
        
        // if compare(price_origin, price_input) {
        //     break;
        // }
        
        // println!("Price is not enough! Tried count : {tried_cnt}");

    }

    (price_input, tried_cnt)
}

fn calculate_coin_amount(price_change: f64, cash_box: &HashMap<usize, i32>) -> (bool, HashMap<usize, i32>) {
    let coin_array = [2.00, 1.00, 0.50, 0.20, 0.10, 0.05, 0.02, 0.01];
    let mut sorted_cash_box: Vec<_> = cash_box.iter().collect();
    sorted_cash_box.sort_by(|a, b| b.0.cmp(a.0));

    let mut price_change = price_change;

    let mut cur_index = 0;

    // let mut result = String::new();

    let mut cash_updated = HashMap::new();
    for (key, value) in cash_box {
        cash_updated.insert(*key, *value);
    }

    if price_change < 0.0 {
        // need to use coins in cash_box
        price_change = round::ceil(0.0 - price_change, 2);

        while price_change > 0.0 {
            println!("{price_change}");

            if cur_index > coin_array.len() - 1 && price_change > 0.0 {
                // initialize cash_updated as cash_box
                for (key, value) in cash_box {
                    let count = cash_updated.entry(*key).or_insert(0);
                    *count = *value;
                }

                return (false, cash_updated);
            }

            if price_change < coin_array[cur_index] - 0.001 {
                if cur_index == coin_array.len() - 1 {
                    break;
                }
                cur_index += 1;
                continue;
            }
    
            let rest_price = (price_change*100.0) as usize;
            let coin_unit = (coin_array[cur_index]*100.0) as usize;
    
            let mut coin_cnt = rest_price / coin_unit;

            let available_cnt = *sorted_cash_box[cur_index].1;
            if available_cnt <= coin_cnt as i32 {
                coin_cnt = available_cnt as usize;                
            }
            let count = cash_updated.entry((coin_array[cur_index] * 100.0) as usize).or_insert(0);
            *count -= coin_cnt as i32;

            // result.insert_str(result.len(), format!("{} coin X {coin_cnt}\n", coin_array[cur_index]).as_str());
            let rest = rest_price - coin_cnt * coin_unit;
            // println!("{}, {}, {}", coin_unit, coin_cnt, rest);
    
            price_change = (rest as f64) / 100.0;
            cur_index += 1;
        }
    }

    else {
        // need to add coins to cash_box
        while price_change > 0.0 {

            if cur_index > coin_array.len() - 1 && price_change > 0.0 {
                // initialize cash_updated as cash_box
                for (key, value) in cash_box {
                    let count = cash_updated.entry(*key).or_insert(0);
                    *count = *value;
                }

                return (false, cash_updated);
            }

            if price_change < coin_array[cur_index] - 0.001 {
                if cur_index == coin_array.len() - 1 {
                    break;
                }
                cur_index += 1;
                continue;
            }
    
            let rest_price = (price_change*100.0) as usize;
            let coin_unit = (coin_array[cur_index]*100.0) as usize;
    
            let mut coin_cnt = rest_price / coin_unit;

            let available_cnt = 50 - sorted_cash_box[cur_index].1;
            if available_cnt <= coin_cnt as i32 {
                coin_cnt = available_cnt as usize;                
            }
            let count = cash_updated.entry((coin_array[cur_index] * 100.0) as usize).or_insert(0);
            *count += coin_cnt as i32;

            // result.insert_str(result.len(), format!("{} coin X {coin_cnt}\n", coin_array[cur_index]).as_str());
            let rest = rest_price - coin_cnt * coin_unit;
            // println!("{}, {}, {}", coin_unit, coin_cnt, rest);
    
            price_change = (rest as f64) / 100.0;
            cur_index += 1;
        }
    }

    return (true, cash_updated);
}

struct Product {
    product_no: usize,
    product_name: String,
    product_price: f64,
}

impl Product {
    fn new(product_no: usize, product_name: String, product_price: f64) -> Product {
        Product {
            product_no,
            product_name,
            product_price,
        }
    }
}

fn generate_product_list(product_cnt: usize) -> Vec<Product> {
    let mut index = 0;

    let mut result: Vec<Product> = Vec::new();

    while index < product_cnt {
        index += 1;
        result.push(
            Product::new(
                index,
                generate_name(),
                generate_price(),
            )
        );
    }

    return result;
}

fn input_product_number(limit: usize) -> usize {

    let mut str_input = String::new();
    let mut product_no: usize = 0;

    loop {
        io::stdin()
            .read_line(&mut str_input)
            .expect("Failed to read line");
        
            product_no = match str_input.trim().parse() {
            Ok(num) => {
                if num > limit || num < 1 {
                    println!("{} Error: Input Correct Number", str_input);
                    str_input = String::from("");
                    continue;
                }
                num
            },
            Err(_) => {
                println!("{} Error: Input Correct Number", str_input);
                str_input = String::from("");
                continue;
            }
        };
        break;
    }

    product_no
}

fn main() {

    let coin_array = [2.00, 1.00, 0.50, 0.20, 0.10, 0.05, 0.02, 0.01];
    let mut cash_box = HashMap::new();
    for coin in coin_array {
        let coin = (coin*100.0) as usize;
        cash_box.insert(coin, 20);
    }

    for (key, value) in &cash_box {
        println!("{:.2} coin : {}", (*key as f64) / 100.0, value);
    }
    println!("------------------------------------------------------");

    let product_list = generate_product_list(10);
    for product in &product_list {
        println!("{} : {} : {}", product.product_no, product.product_name, product.product_price);
    }
    println!("Please input product number : ");
    let product_no = input_product_number(10);

    let selected_product = product_list.get(product_no - 1).unwrap();
    let price_origin = selected_product.product_price;
    let product_name = String::from(selected_product.product_name.as_str());
    println!("You selected {} : {} : {}", product_no, product_name, price_origin);
    println!("------------------------------------------------------");

    loop {
        let (price_input, tried_cnt) = input_payment(price_origin);

        let price_change = price_input - price_origin;
        println!("Change is {:.2}", price_change);

        let (possibility, result) = calculate_coin_amount(price_change, &cash_box);
        println!("{possibility}");

        for (key, value) in result {
            println!("{:.2} coin : {}", (key as f64) / 100.0, value);
        }
        println!("------------------------------------------------------");
        if possibility {
            break;
        }
    }
}