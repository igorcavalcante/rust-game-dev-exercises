use std::io::stdin;

enum VisitorAction {
    Accept,
    AcceptWithNote { note: String },
    Refuse,
    Probation,
}

struct Visitor {
    name: String,
    action: VisitorAction,
    greeting: String,
    age: i8,
}

impl Visitor {
    fn new(name: &str, greeting: &str, action: VisitorAction, age: i8) -> Self {
        Self {
            name: name.trim().to_lowercase(),
            greeting: greeting.to_string(),
            action,
            age,
        }
    }

    fn greet(&self) {
        if self.age < 18 {
            println!("Não pode beber viu?");
        }
        println!("{}", self.greeting)
    }
}

fn get_name() -> String {
    let mut name = String::new();

    stdin()
        .read_line(&mut name)
        .expect("Something went wrong when typing the name");

    name.trim().to_lowercase()
}

fn main() {
    let mut invited_guests = vec![
        Visitor::new("Marcelo", "Tome um chumarrão", VisitorAction::Accept, 31),
        Visitor::new("Eduardo", "Coma um churrasco", VisitorAction::Accept, 29),
        Visitor::new(
            "Pedro",
            "Bem vindo",
            VisitorAction::AcceptWithNote {
                note: String::from("Beba um café antes de entrar"),
            },
            35,
        ),
        Visitor::new("Jean", "Vamos tomar uma?", VisitorAction::Accept, 17),
        Visitor::new(
            "Igor",
            "O que você está fazendo aqui?",
            VisitorAction::Refuse,
            38,
        ),
    ];

    loop {
        println!("Please enter your name or leave blank to quit.");

        let name = get_name();

        if name.is_empty() {
            break;
        } else {
            let known_visitor = invited_guests
                .iter()
                .find(|known_visitor| known_visitor.name == name);

            match known_visitor {
                Some(visitor) => match &visitor.action {
                    VisitorAction::Refuse => println!("You are not allowed to be here."),
                    VisitorAction::AcceptWithNote { note } => {
                        visitor.greet();
                        println!("{}", note);
                    }
                    _ => visitor.greet(),
                },
                None => {
                    println!("Your name is not in the list, you are entering as probatory guest");
                    invited_guests.push(Visitor::new(
                        &name,
                        "Welcome New guest",
                        VisitorAction::Probation,
                        0,
                    ))
                }
            }
        }
    }
}
