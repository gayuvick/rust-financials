use std::collections::HashMap;
use std::io;

#[derive(Debug)]
struct Product {
    name: String,
    price: f64,
}

#[derive(Debug)]
struct Person {
    name: String,
    balance: f64,
    due_amount: f64,
}

impl Person {
    fn new(name: &str, balance: f64) -> Self {
        Person {
            name: name.to_string(),
            balance,
            due_amount: 0.0,
        }
    }

    fn buy_product(&mut self, product: &Product, pay_now: bool, interest_rate: f64, time: f64) {
        if pay_now {
            if self.balance >= product.price {
                self.balance -= product.price;
                println!("{} bought {} for ${:.2}. Remaining balance: ${:.2}", 
                    self.name, product.name, product.price, self.balance);
            } else {
                println!("{} does not have enough balance to buy {}.", self.name, product.name);
            }
        } else {
            let interest = product.price * (interest_rate / 100.0) * time;
            let total_due = product.price + interest;
            self.due_amount += total_due;
            println!("{} bought {} on credit. Due amount: ${:.2} (including interest)", 
                self.name, product.name, total_due);
        }
    }

    fn pay_due(people: &mut Vec<Person>) {
      if people.is_empty() {
          println!("No people found.");
          return;
      }
  
      println!("People with dues:");
      for (index, person) in people.iter().enumerate() {
          if person.due_amount > 0.0 {
              println!("{}. {} - Due: ${:.2}", index + 1, person.name, person.due_amount);
          }
      }
  
      println!("Enter your name:");
      let mut name = String::new();
      io::stdin().read_line(&mut name).expect("Failed to read input");
      let name = name.trim();
  
      let person = people.iter_mut().find(|p| p.name == name);
      
      if let Some(person) = person {
          if person.due_amount <= 0.0 {
              println!("You have no due amount.");
              return;
          }
  
          println!("Your current due: ${:.2}", person.due_amount);
          println!("Enter amount to pay:");
  
          let mut amount = String::new();
          io::stdin().read_line(&mut amount).expect("Failed to read input");
          let amount: f64 = match amount.trim().parse() {
              Ok(val) => val,
              Err(_) => {
                  println!("Invalid amount entered.");
                  return;
              }
          };
  
          if amount > person.due_amount {
              println!("You can't pay more than your due amount (${:.2}).", person.due_amount);
              return;
          }
  
          person.due_amount -= amount;
          println!("Payment successful! New due amount for {}: ${:.2}", person.name, person.due_amount);
      } else {
          println!("Person not found.");
      }
  }
  

   
}

fn main() {
    let mut persons: HashMap<String, Person> = HashMap::new();
    persons.insert("Alice".to_string(), Person::new("Alice", 500.0));
    persons.insert("Bob".to_string(), Person::new("Bob", 1000.0));

    let products = vec![
        Product {
            name: "Laptop".to_string(),
            price: 800.0,
        },
        Product {
            name: "Phone".to_string(),
            price: 500.0,
        },
    ];

    loop {
        println!("\nAvailable actions:");
        println!("1. Buy a product");
        println!("2. Pay due amount");
        println!("3. Show balances");
        println!("4. Exit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let choice = input.trim();

        match choice {
            "1" => {
                println!("Enter person name (Alice/Bob):");
                input.clear();
                io::stdin().read_line(&mut input).unwrap();
                let person_name = input.trim().to_string();

                if let Some(person) = persons.get_mut(&person_name) {
                    println!("Available products:");
                    for (i, product) in products.iter().enumerate() {
                        println!("{}. {} - ${:.2}", i + 1, product.name, product.price);
                    }

                    println!("Enter product number:");
                    input.clear();
                    io::stdin().read_line(&mut input).unwrap();
                    let product_index: usize = match input.trim().parse::<usize>() {
                        Ok(num) if num > 0 && num <= products.len() => num - 1,
                        _ => {
                            println!("Invalid product selection.");
                            continue;
                        }
                    };

                    let product = &products[product_index];

                    println!("Pay now? (yes/no):");
                    input.clear();
                    io::stdin().read_line(&mut input).unwrap();
                    let pay_now = input.trim().eq_ignore_ascii_case("yes");

                    if pay_now {
                        person.buy_product(product, true, 0.0, 0.0);
                    } else {
                        println!("Enter interest rate (%):");
                        input.clear();
                        io::stdin().read_line(&mut input).unwrap();
                        let interest_rate: f64 = match input.trim().parse() {
                            Ok(rate) if rate > 0.0 => rate,
                            _ => {
                                println!("Invalid interest rate.");
                                continue;
                            }
                        };

                        println!("Enter time (years):");
                        input.clear();
                        io::stdin().read_line(&mut input).unwrap();
                        let time: f64 = match input.trim().parse() {
                            Ok(t) if t > 0.0 => t,
                            _ => {
                                println!("Invalid time.");
                                continue;
                            }
                        };

                        person.buy_product(product, false, interest_rate, time);
                    }
                } else {
                    println!("Person not found.");
                }
            }
            "2" => {
              let mut people: Vec<Person> = persons.drain().map(|(_, p)| p).collect();
              Person::pay_due(&mut people);
              for person in people {
                  persons.insert(person.name.clone(), person);
              }

            }
            "3" => {
                println!("\nCurrent Balances and Dues:");
                for person in persons.values() {
                    println!(
                        "{} - Balance: ${:.2}, Due Amount: ${:.2}",
                        person.name, person.balance, person.due_amount
                    );
                }
            }
            "4" => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid option! Try again."),
        }
    }
}
