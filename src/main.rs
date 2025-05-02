#[derive(Debug, Clone, Copy)]
struct Cubsat {
    cub_id: u32,
}


#[derive(Debug, Clone, Copy)]
enum StatusMessage {
    Ok,
}

fn check_status(cub_id:Cubsat) -> StatusMessage{
    StatusMessage::Ok
}

fn  main() {
    let cub_id = Cubsat { cub_id: 1 };
    let status = check_status(cub_id.clone());
    println!("{:?}", status.clone());

    let status = check_status(cub_id);
     println!("{:?}", status);
}
