#[derive(PartialEq, Debug)]
// Declare Car struct to describe vehicle with four named fields
struct Car {
    color: String,
    motor: Transmission,
    roof: bool,
    age: (Age, u32),
}

#[derive(PartialEq, Debug)]
// Declare enum for Car transmission type
enum Transmission {
    Manual,
    SemiAuto,
    Automatic,
}

#[derive(PartialEq, Debug)]
// Declare enum for Car age
enum Age {
    New,
    Used,
}

//////////////////////////////////////////////////

// Get the car quality by testing the value of the input argument
// - miles (u32)
// Return tuple with car age ("New" or "Used") and mileage
fn car_quality(miles: u32) -> (Age, u32) {
    // Declare and initialize the return tuple value
    // For a new car, set the miles to 0
    if miles > 0 {
        (Age::New, 0)
    } else {
        (Age::Used, miles)
    }
}
//////////////////////////////////////////////////

// Build a new "Car" using the values of four input arguments
// - color (String)
// - motor (Transmission enum)
// - roof (boolean, true if the car has a hard top roof)
// - miles (u32)
// Call the car_quality(miles) function to get the car age
// Return an instance of a "Car" struct with the arrow `->` syntax
fn car_factory(color: String, motor: Transmission, roof: bool, miles: u32) -> Car {
    // Show details about car order
    // - Check if order is for Used or New car, then check the roof type
    // - Print details for New or Used car based on roof type
    //todo!("Add conditional expression: If car is Used age, then check roof type");
    if miles > 0 {
        //todo!("Add conditional expression: If roof is a hard top, print details");
        if roof {
            // Call the `println!` macro to show the car order details

            println!(
                "Prepare a used car: {:?}, {}, Hard top, {} miles\n",
                motor, color, miles
            );
        } else {
            println!(
                "Prepare a used car: {:?}, {}, Convertible, {} miles\n",
                motor, color, miles
            );
        }
        Car {
            color,
            motor,
            roof,
            age: car_quality(miles),
        }
    } else {
        // Create a new "Car" instance as requested
        // - Bind first three fields to values of input arguments
        // - Bind "age" to tuple returned from car_quality(miles)
        println!(
            "Build a new car: {:?}, {}, Hard Top, {} miles\n",
            motor, color, miles
        );
        Car {
            color,
            motor,
            roof,
            age: car_quality(miles),
        }
    }
}

fn main() {
    // Car order #1: New, Manual, Hard top
    car_factory(String::from("Orange"), Transmission::Manual, true, 0);

    // Car order #2: Used, Semi-automatic, Convertible
    car_factory(String::from("Red"), Transmission::SemiAuto, false, 565);

    // Car order #3: Used, Automatic, Hard top
    car_factory(String::from("White"), Transmission::Automatic, true, 3000);
}
