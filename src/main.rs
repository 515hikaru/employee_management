use std::io;
use std::collections::HashMap;

#[derive(Debug)]
enum OrderType{
    Add(AddOrder),
    List(ListOrder),
}

#[derive(Debug)]
struct AddOrder {
    name: String,
    department: String,
}

impl AddOrder {
    fn exec(self, mut map: HashMap<String, Vec<String>>) -> HashMap<String, Vec<String>> {
        let department = self.department.clone();
        let mut vec2 = {
            let vec = map.entry(department).or_insert(Vec::new());
            let mut vec2 = Vec::new();
            for data in vec.iter() {
                vec2.push(data.to_string());
            }
            vec2
        };
        vec2.push(self.name);
        map.insert(self.department, vec2.to_vec());
        map
    }
}

#[derive(Debug)]
struct ListOrder {
    department: Option<String>,
}

impl ListOrder {
    fn exec(&self, map: &HashMap<String, Vec<String>>) {
        println!("Not Implemented.");
    }
}

fn main() {
    let mut employees: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        let mut s = String::new();
        io::stdin().read_line(&mut s).expect("failed to read line.");
        let words: Vec<&str> = s.split_whitespace().collect();
        let order = parse_line(&words);
        println!("{:?}", order);
        match order {
            None => println!("エラーが発生しました、やり直してください"),
            Some(ord) => match ord {
                OrderType::Add(add) => employees = add.exec(employees),
                OrderType::List(list) => list.exec(&employees),
            }
        }
        println!("map: {:?}", employees);
    }
}

fn parse_line(words: &Vec<&str>) -> Option<OrderType> {
    if words[0] == "Add" {
        let add_order = parse_add(&words);
        match add_order {
            Some(order) => Some(OrderType::Add(order)),
            None => None
        }
    } else if words[0] == "List" {
        println!("poyo");
        None
    } else {
        println!("存在しない命令です: {}", words[0]);
        None
    }
}

fn parse_add(order: &Vec<&str>) -> Option<AddOrder> {
    if order.len() != 4 {
        return None
    }
    if order[2] != "to" {
        return None
    }
    Some(AddOrder { name: order[1].to_string(), department: order[3].to_string()})
}
