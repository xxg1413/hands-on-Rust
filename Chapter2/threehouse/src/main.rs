use std::io::stdin;

#[derive(Debug)]
struct Visitor {
    name: String,
    action: VisitorAction,
    age: i8,
}

#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote { note: String },
    Refuse,
    Probation,
}



impl Visitor {
    fn new(name: &str, action: VisitorAction, age: i8) -> Self {
        Self {
            name: name.to_lowercase(),
            action,
            age
        }
    }

    fn greet_visitor(&self) {
        match &self.action {
            VisitorAction::Accept => println!("欢迎来到组织,{}",self.name),
            VisitorAction::AcceptWithNote { note} => {
                println!("你好，{}，欢迎你", self.name);
                println!("{}", note);
                if self.age < 21 {
                    println!("{},小朋友，不要来这里玩", self.name);
                }
            },

            VisitorAction::Probation => println!("你已经是一个预备成员了，{}",self.name),
            VisitorAction::Refuse => println!("你不允许加入组织，{}", self.name),
        }
    }
}

fn get_name() -> String {

    let mut name = String::new();
    stdin()
        .read_line(&mut name)
        .expect("File to read line");

    name
        .trim()
        .to_lowercase()
}

fn main() {

    //let visitor_list = ["pxiaoer","test1","test2","测试3"];
    let mut visitor_list = vec![
        Visitor::new("pxiaoer", VisitorAction::Accept,45),
        Visitor::new("test1",VisitorAction::AcceptWithNote {
            note: String::from("这是一个测试id")
        }, 15),
        Visitor::new("测试2",VisitorAction::Refuse, 30),
    ];

    loop {
        println!("你好，请输入你的名字：");
        let name = get_name();
        //println!("你好，{:?}", name);
        //let mut allow_them_in = false;

        let know_visitor = visitor_list.iter().find(|visitor| visitor.name == name);

        match know_visitor {
            Some(visitor) => visitor.greet_visitor(),
            None => {
                if name.is_empty() {
                    break;
                } else {
                    println!("{},好像是新朋友",name);
                    visitor_list.push(Visitor::new(&name, VisitorAction::AcceptWithNote{
                        note: String::from("这是一个新朋友")},0)
                    );
                }
            }
        }
    }
}





