mod lambda;
mod number;
mod object;

use lambda::LambdaRegistry;
use object::NumberObject;

fn main() {
    let mut registry = LambdaRegistry::new();
    registry.define("even", number::is_even);
    registry.define("odd", number::is_odd);
    registry.define("prime", number::is_prime);

    let values: Vec<i64> = std::env::args()
        .skip(1)
        .filter_map(|arg| arg.parse::<i64>().ok())
        .collect();

    let values = if values.is_empty() {
        vec![1, 2, 3, 4, 5, 9, 11, 12, 17, 21]
    } else {
        values
    };

    println!("lambdaGen number logic");
    println!("=======================");

    for value in values {
        let object = NumberObject::new(value);
        let even = registry.call("even", value).unwrap_or(false);
        let odd = registry.call("odd", value).unwrap_or(false);
        let prime = registry.call("prime", value).unwrap_or(false);

        println!(
            "{value}: even={even}, odd={odd}, prime={prime} | {}",
            object.sentence()
        );
    }
}
