trait Engine {
    fn new() -> Self
    where
        Self: Sized;
    fn fuel_type(&self) -> String;
}

struct DesielEngine {
    fuel_type: String,
}

impl Engine for DesielEngine {
    fn new() -> Self {
        DesielEngine {
            fuel_type: String::from("Diesel"),
        }
    }

    fn fuel_type(&self) -> String {
        self.fuel_type.clone()
    }
}

// New PetrolEngine struct
struct PetrolEngine {
    fuel_type: String,
}

// Implement the Engine trait for PetrolEngine
impl Engine for PetrolEngine {
    fn new() -> Self {
        PetrolEngine {
            fuel_type: String::from("Petrol"),
        }
    }

    fn fuel_type(&self) -> String {
        self.fuel_type.clone()
    }
}

struct Car {
    color: String,
    engine: Box<dyn Engine>,
    wheels: i32,
}

impl Car {
    // Allow specifying the engine type dynamically
    fn new(color: String, engine: Box<dyn Engine>, wheels: i32) -> Self {
        Car {
            color,
            engine,
            wheels,
        }
    }

    fn display(&self) {
        println!("Car Color: {}", self.color);
        println!("Car Engine Fuel Type: {}", self.engine.fuel_type());
        println!("Car Wheels: {}", self.wheels);
    }
}

fn main() {
    // Create a car with a diesel engine
    let diesel_car = Car::new(String::from("Red"), Box::new(DesielEngine::new()), 4);
    diesel_car.display();

    // Create a car with a petrol engine
    let petrol_car = Car::new(String::from("Blue"), Box::new(PetrolEngine::new()), 4);
    petrol_car.display();
}