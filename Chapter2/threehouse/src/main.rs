use std::io::stdin;

#[derive(Debug)]
struct Visitor {
    name: String,
    greeting: String,
}


impl Visitor {
    fn new(name: &str, greeting: &str) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
        }
    }

    fn greet_visitor(&self) {
        println!("{}",self.greeting);
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
        Visitor::new("pxiaoer", "你真的很优秀，P小二"),
        Visitor::new("test1","这只是一个测试"),
        Visitor::new("测试2","这又是一个测试，不要太难过"),
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
                    visitor_list.push(Visitor::new(&name, "新朋友"));
                }
            }
        }
    }
}





