#[allow(unused_variables)]

#[derive(Debug)]
struct Cubesat{
    id:u64,
}

#[derive(Debug)]
enum Status {
    Ok,
}

fn check_status(sat_id: Cubesat) -> Status {
    Status::Ok
}

fn main() {
    let cubsat_a = Cubesat { id: 1 };
    let cubsat_b = Cubesat { id: 2 };
    let cubsat_c = Cubesat { id: 3 };

    //println!("a: {:?} , b: {:?}, c: {:?}", cubsat_a , cubsat_b, cubsat_c);

    let a_status = check_status(cubsat_a);
   let b_status = check_status(cubsat_b);
   let c_status = check_status(cubsat_c);
    println!("a: {:?}, b: {:?}, c: {:?}", a_status, b_status, c_status);

    
    let a_status = check_status(cubsat_a);
   let b_status = check_status(cubsat_b);
   let c_status = check_status(cubsat_c);
    println!("a: {:?} , b: {:?}, c: {:?}", cubsat_a , cubsat_b, cubsat_c);
 
    
}