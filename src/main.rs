use std::io;

struct Contacts {
    name: String,
    number: u32,
}

fn main() {
    loop{
        println!("---------------------------------------------------------------------------------------------------------------------");
        println!("---------------------------------- Press '1' to add contacts up to five contacts. -----------------------------------");
        println!("-------------------------------- Press '0' or or 'ctrl + c' to exit this application. -------------------------------");
        println!("---------------------------------------------------------------------------------------------------------------------");
        let mut a = String::new();
        io::stdin().read_line(&mut a).expect("--- Failed to read line. ---");
        let b: u32 = match a.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("---------------");
                println!("Enter a number."); continue
            },
        };

        if b == 1   {
            create1();
            break
        }   else if b == 0  {
            break
        }   else    {
                println!("----------------------------------------------");
                println!("Please enter a valid number, either 1, 2 or 0.");
                continue
        }

    }
}

fn create1() {
    loop{
        println!("------------------------------");
        println!("Enter your first contact name.");
        println!("------------------------------");
        let mut a = String::new();
        io::stdin().read_line(&mut a).expect("--- Failed to read line. ---");
        let a = a.trim();
        println!("--------------------------------------");
        println!("Enter your first contact phone number.");
        println!("--------------------------------------");
        let mut b = String::new();
        io::stdin().read_line(&mut b).expect("--- Failed to read line. ---");
        let b: u32 = match b.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("------------------------------------------");
                println!("Enter a number. Re-input the contact info."); continue
            },
        };
        
        let contact1 = Contacts {
            name: a.to_string(),
            number: b,
        };

        println!("------------------------------------------");
        println!("You created the contact: {}", contact1.name);
        println!("Their phone number is: {}", contact1.number);
        println!("------------------------------------------");
        println!("Press 1 to add another contact. Press 2 to return to the home screen.");
        println!("---------------------------------------------------------------------");
        let mut input1 =  String::new();
        io::stdin().read_line(&mut input1).expect("--- Failed to read line. ---");
        let input1: u32 = match input1.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("-------------------------------------------------------");
                println!("Enter a number only. Re-input your contact information."); continue
            },
        };
        
        if input1 == 1  {
            create2();
        } else if input1 == 2   {
            main();
        } else  {
            println!("-------------------------------------------");
            println!("Please enter a valid number, either 1 or 2.");
            println!("-------------------------------------------");
            continue
        }
    }
}

fn create2() {
    loop{
        println!("-------------------------------");
        println!("Enter your second contact name.");
        println!("-------------------------------");
        let mut c = String::new();
        io::stdin().read_line(&mut c).expect("--- Failed to read line. ---");
        let c = c.trim();
        println!("---------------------------------------");
        println!("Enter your second contact phone number.");
        println!("---------------------------------------");
        let mut d = String::new();
        io::stdin().read_line(&mut d).expect("--- Failed to read line. ---");
        let d: u32 = match d.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("------------------------------------------");
                println!("Enter a number. Re-input the contact info."); continue
            },
        };
        
        let contact2 = Contacts {
            name: c.to_string(),
            number: d,
        };

        println!("------------------------------------------");
        println!("You created the contact: {}", contact2.name);
        println!("Their phone number is: {}", contact2.number);
        println!("------------------------------------------");
        println!("Press 1 to add another contact. Press 2 to return to the home screen.");
        println!("---------------------------------------------------------------------");
        let mut input2 =  String::new();
        io::stdin().read_line(&mut input2).expect("--- Failed to read line. ---");
        let input2: u32 = match input2.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if input2 == 1  {
            create3();
        } else if input2 == 2   {
            main();
        } else  {
            println!("-------------------------------------------");
            println!("Please enter a valid number, either 1 or 2.");
            println!("-------------------------------------------");
            continue
        }
    }
}

fn create3() {
    loop{
        println!("------------------------------");
        println!("Enter your third contact name.");
        println!("------------------------------");
        let mut e = String::new();
        io::stdin().read_line(&mut e).expect("--- Failed to read line. ---");
        let e = e.trim();
        println!("--------------------------------------");
        println!("Enter your third contact phone number.");
        println!("--------------------------------------");
        let mut f = String::new();
        io::stdin().read_line(&mut f).expect("--- Failed to read line. ---");
        let f: u32 = match f.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("------------------------------------------");
                println!("Enter a number. Re-input the contact info."); continue
            },
        };
        
        let contact3 = Contacts {
            name: e.to_string(),
            number: f,
        };

        println!("------------------------------------------");
        println!("You created the contact: {}", contact3.name);
        println!("Their phone number is: {}", contact3.number);
        println!("------------------------------------------");
        println!("Press 1 to add another contact. Press 2 to return to the home screen.");
        println!("---------------------------------------------------------------------");
        let mut input3 =  String::new();
        io::stdin().read_line(&mut input3).expect("--- Failed to read line. ---");
        let input3: u32 = match input3.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if input3 == 1  {
            create4();
        } else if input3 == 2   {
            main();
        } else  {
            println!("-------------------------------------------");
            println!("Please enter a valid number, either 1 or 2.");
            println!("-------------------------------------------");
            continue
        }
    }
}

fn create4() {
    loop{
        println!("-------------------------------");
        println!("Enter your fourth contact name.");
        println!("-------------------------------");
        let mut g = String::new();
        io::stdin().read_line(&mut g).expect("--- Failed to read line. ---");
        let g = g.trim();
        println!("---------------------------------------");
        println!("Enter your fourth contact phone number.");
        println!("---------------------------------------");
        let mut h = String::new();
        io::stdin().read_line(&mut h).expect("--- Failed to read line. ---");
        let h: u32 = match h.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        let contact4 = Contacts {
            name: g.to_string(),
            number: h,
        };

        println!("------------------------------------------");
        println!("You created the contact: {}", contact4.name);
        println!("Their phone number is: {}", contact4.number);
        println!("------------------------------------------");
        println!("Press 1 to add another contact. Press 2 to return to the home screen.");
        println!("---------------------------------------------------------------------");
        let mut input4 =  String::new();
        io::stdin().read_line(&mut input4).expect("--- Failed to read line. ---");
        let input4: u32 = match input4.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("------------------------------------------");
                println!("Enter a number. Re-input the contact info."); continue
            },
        };

        if input4 == 1  {
            create5();
        } else if input4 == 2   {
            main();
        } else  {
            println!("-------------------------------------------");
            println!("Please enter a valid number, either 1 or 2.");
            println!("-------------------------------------------");
            continue
        }
    }
}

fn create5() {
    loop{
        println!("------------------------------");
        println!("Enter your fifth contact name.");
        println!("------------------------------");
        let mut i = String::new();
        io::stdin().read_line(&mut i).expect("--- Failed to read line. ---");
        let i = i.trim();
        println!("--------------------------------------");
        println!("Enter your fifth contact phone number.");
        println!("--------------------------------------");
        let mut j = String::new();
        io::stdin().read_line(&mut j).expect("--- Failed to read line. ---");
        let j: u32 = match j.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("------------------------------------------");
                println!("Enter a number. Re-input the contact info."); continue
            },
        };
        
        let contact5 = Contacts {
            name: i.to_string(),
            number: j,
        };

        println!("------------------------------------------");
        println!("You created the contact: {}", contact5.name);
        println!("Their phone number is: {}", contact5.number);
        println!("------------------------------------------");
        println!("Reached the max amount of contacts, returning to home page.");
        println!("---------------------------------------------------------------------");
        main();
    }
}