enum Status {
    Selected(String),
    InProgress(String),
    Done(String),
}

impl Status {
    fn print_status(&self) {
        match self {
            Status::Selected(t) => println!("Selected_{}", t),
            Status::InProgress(t) => println!("InProgress_{}", t),
            Status::Done(t) => println!("Done_{}", t),
        }
    }
}

fn main() {
    let s1 = Status::Selected(String::from("monday"));
    s1.print_status();
    let option_s1 = Some(s1);
    let option_s2 = status_transition(option_s1);
    if let Some(s2) = option_s2 {
        s2.print_status();
    }
}

fn status_transition(current: Option<Status>) -> Option<Status> {
    match current {
        None => None,
        Some(s) => match s {
            Status::Selected(t) => Some(Status::InProgress(t)),
            Status::InProgress(t) => Some(Status::Done(t)),
            Status::Done(t) => Some(Status::Done(t)),
        },
    }
}
