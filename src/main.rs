use std::collections::HashMap;
use std::io;

#[derive(Debug)]
enum OrderType {
    Add(AddOrder),
    List(ListOrder),
}

#[derive(Debug)]
struct AddOrder {
    name: String,
    department: String,
}

impl AddOrder {
    fn exec(self, map: &mut HashMap<String, Vec<String>>) {
        // department を 2 回使いたいのでコピーしておく
        let department = self.department.clone();
        // そのままやると2つの可変参照を使用する
        // 現在のベクトルを取得する部分をブロック式にすることでブロック内での可変参照をひとつに
        let mut vec2 = {
            let vec = map.entry(department).or_insert(Vec::new());
            let mut vec2 = Vec::new();
            for data in vec.iter() {
                vec2.push(data.to_string());
            }
            vec2.sort_unstable();
            vec2
        };
        vec2.push(self.name);
        map.insert(self.department, vec2.to_vec());
    }
}

#[derive(Debug)]
struct ListOrder {
    department: String,
}

impl ListOrder {
    fn exec(&self, map: &HashMap<String, Vec<String>>) {
        // 全社員を出力する場合
        if self.department == "All".to_string() {
            for (key, _) in map.iter() {
                let mut vec: Vec<String> = match map.get(key) {
                    Some(v) => v.to_vec(),
                    None => Vec::new(),
                };
                self.print_names(key, &vec);
            }
            return ();
        }
        let vec: Vec<String> = match map.get(&self.department) {
            Some(v) => v.to_vec(),
            None => Vec::new(),
        };
        // 指定された部署が存在しない場合
        if vec.len() == 0 {
            println!("No one is registered at {}", self.department);
            return ();
        }
        // 指定された部署のみ出力する場合
        self.print_names(&self.department, &vec);
    }

    fn print_names(&self, department: &String, names: &Vec<String>) {
        print!("Department {} menber:", department);
        for name in names.iter() {
            print!(" {}", name);
        }
        println!("");
    }
}

fn main() {
    let mut employees: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        let mut s = String::new();
        io::stdin().read_line(&mut s).expect("failed to read line.");
        let words: Vec<&str> = s.split_whitespace().collect();
        let order = parse_line(&words);
        match order {
            None => println!("エラーが発生しました、やり直してください"),
            Some(ord) => match ord {
                OrderType::Add(add) => add.exec(&mut employees),
                OrderType::List(list) => list.exec(&employees),
            },
        }
    }
}

fn parse_line(words: &Vec<&str>) -> Option<OrderType> {
    if words[0] == "Add" {
        let add_order = parse_add(&words);
        match add_order {
            Some(order) => Some(OrderType::Add(order)),
            None => None,
        }
    } else if words[0] == "List" {
        match parse_list(&words) {
            Some(order) => Some(OrderType::List(order)),
            None => None,
        }
    } else {
        println!("存在しない命令です: {}", words[0]);
        None
    }
}

fn parse_add(order: &Vec<&str>) -> Option<AddOrder> {
    if order.len() != 4 {
        return None;
    }
    if order[2] != "to" {
        return None;
    }
    Some(AddOrder {
        name: order[1].to_string(),
        department: order[3].trim().to_string(),
    })
}

fn parse_list(order: &Vec<&str>) -> Option<ListOrder> {
    if order.len() != 2 {
        return None;
    }
    Some(ListOrder {
        department: order[1].trim().to_string(),
    })
}
