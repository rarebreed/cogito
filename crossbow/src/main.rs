use clap::Command;

fn test() {
    let r1 = vec!["a", "b", "c"];
    let r2 = 0..r1.len();
    let zipped = r1.iter().zip(r2);
    for (item, index) in zipped {
        println!("{index} -> {item}");
    }
}

#[derive(Debug)]
pub struct Equipment {
    pub name: String,
    pub price: f64,
    pub weight: f64
}

pub struct Character {
    name: String,
    equipment: Vec<Equipment>
}

// These look like methods, but there is a crucial difference.  
// - They do not all implicitly have &self as an argument
// - The implementation of the methods can happen separately from the struct declaration (even in another file!)
impl Character {
    pub fn new(name: String) -> Self {
        Character { name , equipment: vec![]}
    }

    pub fn add_equipment<'a>(&'a mut self, item: Equipment) -> &'a Vec<Equipment> {
        self.equipment.push(item);
        &self.equipment
    }

    pub fn remove_equipment(&mut self, name: String) {
        let r1 = 0..self.equipment.len();
        let zipped = r1.zip(self.equipment.iter());
        let mut at = 0;
        let mut found = false;
        for (idx, item) in zipped {
            if item.name == name {
                at = idx;
                found = true;
                break
            }
        }
        if found {
            self.equipment.remove(at);
        }
    }
}

fn test2() {
    let mut char = Character::new("Sean".into());
    let equip = char.add_equipment(Equipment {
        name: "sword".into(),
        price: 1000.00,
        weight: 10.0
    });
    println!("{equip:#?}");
}

#[tokio::main]
async fn main() {
    //let _cmd = Command::new("docker demo").about("Build an executable for ");
    test2()
}
