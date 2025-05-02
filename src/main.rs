use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct GroundStation {
    radio_freq:f64,
}

fn main() {
    let base = Rc::new(RefCell::new(GroundStation { radio_freq:87.0}));

    println!("{:?}",base);

    {
        let mut base_2 = base.borrow_mut();
        base_2.radio_freq -= 15.6;
        println!("base_2: {:?}",base_2);
        println!("base: {:?}",base);
    }

    println!("base: {:?}",base);

    
        let mut base_3 = base.borrow_mut();
        base_3.radio_freq += 15.6;
        println!("base_3: {:?}",base_3);
        println!("base: {:?}",base);
    
}