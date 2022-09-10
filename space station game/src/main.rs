use std::io::{stdin,stdout,Write};

#[derive(Clone)]
struct Section {
    health:[i32;2],
    cost:Vec<(String,i32)>,
    update:fn(&mut Station),
    data:Vec<(String,i32)>,
    name:String,
    help:String, 
}

impl Section {
    fn build_section(section:&Section) -> Section {
        Section {health:section.health , cost:vec![] , update:section.update , data:section.data.clone() , name:section.name.clone() , help:section.help.clone()}

    }
}

#[derive(Debug,Clone)]
struct Resource {
    amount:[i32;2],
    name:String, 
}

#[derive(Clone)]
struct Command {
    require:Vec<(String,i32)>,
    function:fn(&mut Game,&mut Vec<String> ,&(String,String)),
    name:String,
}

impl Command {
    fn new(require:Vec<(String,i32)> , function:fn(&mut Game,&mut Vec<String> , &(String,String)) , name:String) -> Command {
        Command { require: require, function: function, name: name }
    }
}

#[derive(Debug,Clone)]
struct Help {
    key:String,
    answer:String,
}

#[derive(Clone)]
struct Station {
    sections:Vec<Section>,
    build_sections:Vec<Section>,
    resources:Vec<Resource>,
    commands:Vec<Command>,
    help:Vec<Help>,
}


impl Station {
    fn empty() -> Station {
        Station {sections:vec![] , resources: vec![], commands:vec![] , help:vec![] , build_sections:vec![]}
    }

    fn resource_manipulate(&mut self , name:&str , amount:i32) {
        let mut index = 0;
        while index < self.resources.len() {
            if self.resources[index].name == name {
                self.resources[index].amount[0] = self.resources[index].amount[0] + amount;
            }
            index = index + 1;
        }
    }

    fn get_resource(&self , name:&str) -> i32 {
        for i in &self.resources {
            if i.name == name {
                return i.amount[0];
            }
        }
        0
    }

    fn update(&mut self) {
        let mut index = 0;
        while index < self.build_sections.len() {
            let mut section = &self.build_sections[index];
            (section.update)(self);
            index = index + 1;
        }
    }

}


#[derive(Clone)]
struct Game {
    day:f64,
    station:Station,  
}

impl Game {
    fn add_content(&mut self) {
        //Sections
        fn solar_update(station:&mut Station) {
            station.resource_manipulate("power",4);
        }
        let mut solar = Section {health:[100,100] , cost:vec![("iron".to_string(),4),("power".to_string(),4)] , data:vec![] , update:solar_update , name:"Solar".to_string() , help:"provides 4 power each day".to_string()};
        self.station.sections.push(solar);
        //Resource
        let mut resource = Resource {amount:[10,10] , name:"power".to_string()};
        self.station.resources.push(resource);
        let mut resource = Resource {amount:[10,10] , name:"iron".to_string()};
        self.station.resources.push(resource);
        //Command
        fn display_resources(data:&mut Game , namespace:&mut Vec<String> , args:&(String,String)) {
            println!("Resource(s):");
            if !(args.1 == "") {
                for i in &data.station.resources {
                    if i.name == args.1 {
                        println!("{}:{:?}",i.name,i.amount);
                    }
                }
            } else {
                for i in &data.station.resources {
                    println!("{}:{:?}",i.name,i.amount);
                }

            }

        }
        let mut command = Command::new(vec![] , display_resources , "re".to_string());
        self.station.commands.push(command);

        fn build_section(data:&mut Game , namespace:&mut Vec<String> , args:&(String,String)) {
            if args.1 == "" {
                println!("Section must be specified.");
                return;
            }
            let mut index = 0;
            while index < data.station.sections.len() {
                if data.station.sections[index].name == args.1 {
                    let mut index_2 = 0;
                    let mut pay = false;
                    let mut re = false;
                    while index_2 < data.station.sections[index].cost.len() {
                        if data.station.get_resource(&data.station.sections[index].cost[index_2].0) < data.station.sections[index].cost[index_2].1 && !pay{
                            re = true;
                            println!("Need {} {} have {} {}",data.station.sections[index].cost[index_2].1,data.station.sections[index].cost[index_2].0,data.station.sections[index].cost[index_2].1,data.station.sections[index].cost[index_2].0);
                        }
                        if pay {
                            data.station.resource_manipulate(&data.station.sections[index].cost[index_2].0.clone(),-data.station.sections[index].cost[index_2].1.clone());
                        }
                        index_2 = index_2 + 1; 
                        if index_2 == data.station.sections[index].cost.len() && !re && !pay {
                            pay = true;
                            index_2 = 0;
                        }
                    }
                    if re {
                        return;
                    }
                    data.station.build_sections.push(Section::build_section(&data.station.sections[index]));
                    println!("Section build Succesfully.");
                    return;
                }
                index = index + 1;
            }
            println!("Unknown Section.");
        }
        let mut command = Command::new(vec![] , build_section , "build".to_string());
        self.station.commands.push(command);

        fn show_sections(data:&mut Game , namespace:&mut Vec<String> , args:&(String,String)) {
            for i in &data.station.build_sections {
                println!("{} >> health:{:?} >> help:{}",i.name,i.health,i.help)
            }
        }
        let mut command = Command::new(vec![] , show_sections , "secb".to_string());
        self.station.commands.push(command);

    }

    fn new() -> Game {
        Game {day:0 as f64 , station:Station::empty()}
    }

    fn start(&mut self) {
        self.add_content();
        let mut namespace = vec!["Main".to_string()];
        loop {
            print!("{esc}c", esc = 27 as char);
            println!("Integrated Control Terminal (ICT) of Satelite F43B");
            println!("Mission Day:{}",&self.day);
            self.station.update();
            self.station.resource_manipulate("power",-1);
            loop {
                print!("[{:?}] << ",&namespace);
                let mut command = String::new();
                let _ = stdout().flush();
                stdin().read_line(&mut command);
                if let Some('\n') = command.chars().next_back() {
                   command.pop();
                }
                if let Some('\r') = command.chars().next_back() {
                    command.pop();
                }
                command.push_str(" ");
                let mut name = command.split_once(" ").unwrap_or(("-","-"));
                if name.0 == "next" {
                    self.day = self.day + 1.0 as f64;
                    break;
                }
                let mut index = 0;
                let mut vec = self.station.commands.clone();
                while index < vec.len() {
                    if vec[index].name == name.0 {
                        let mut temp = name.1.to_string();
                        temp.pop();
                        name.1 = temp.as_ref();

                        (vec[index].function)(self,&mut namespace,&(name.0.to_string(),name.1.to_string()));
                        break;
                    }
                    index = index + 1;
                }
            }
            
            
        }
    } 
}

fn main() {
    let mut game = Game::new();
    game.start();
}
