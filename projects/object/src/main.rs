pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            },
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    pub fn print_list(&self) {
        let output = self.list
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<_>>()
        .join(", ");
        println!("List: {}", output);
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// This implementation uses generics instead of trait objects, so can only contain one type ex: button, textfield, etc.
// pub struct Screen<T: Draw> {
//     pub components: Vec<T>,
// }

// impl<T> Screen<T>
// where
//     T: Draw,
// {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Drawing a button: {}", self.label);
    }
}

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("Drawing a select box with options: {:?}", self.options);
    }
}

fn main() {
    // let mut ac = AveragedCollection { list: Vec::new(), average: 0.0 };
    // ac.add(10);
    // ac.add(20);
    // ac.print_list();
    // println!("Average: {}", ac.average());
    // ac.add(30);
    // ac.add(40);
    // ac.print_list();
    // println!("Average: {}", ac.average());
    // ac.remove();
    // ac.print_list();
    // println!("Average: {}", ac.average());

    let button = Button {
        width: 50,
        height: 10,
        label: String::from("OK"),
    };
    let select_box = SelectBox {
        width: 75,
        height: 20,
        options: vec![String::from("Yes"), String::from("No"), String::from("Maybe")],
    };
    let screen = Screen {
        components: vec![Box::new(button), Box::new(select_box)],
    };
    screen.run();
}
