#![feature(min_specialization)]

mod a {
    use mono_event::{event, listen};

    #[event]
    pub struct SayHi {
        pub name: String,
    }

    #[listen(SayHi)]
    fn print_hi(event: &mut SayHi) {
        println!("teest a hi ");
    }

    #[listen(SayHi)]
    fn print_hmm(event: &mut SayHi) {
        println!("test a hmm");
    }

    #[test]
    fn main() {
        SayHi {
            name: "Bruce".to_string(),
        }
        .dispatch()
        .unwrap();
    }
}

mod b {
    use mono_event::{event, high_priority, listen};

    #[event]
    pub struct SayHi {
        pub name: String,
    }

    #[listen(SayHi)]
    #[low_priority]
    fn print_hi(event: &mut SayHi) {
        println!("teest b hi ");
    }

    #[high_priority]
    #[listen(SayHi)]
    fn print_hmm(event: &mut SayHi) {
        println!("test b hmm");
    }

    #[test]
    fn main() {
        SayHi {
            name: "Bruce".to_string(),
        }
        .dispatch()
        .unwrap();
    }
}
