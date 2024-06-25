use actix_web::{get, web, App, HttpServer, Responder};
use serde::Deserialize;
use std::ops::RangeInclusive;

#[derive(Deserialize)]
struct PrimeQuery {
    min: Option<u64>,
    max: Option<u64>,
}

// Function to check if a number is prime
fn is_prime(num: u64) -> bool {
    if num <= 1 {
        return false;
    }
    let limit = (num as f64).sqrt() as u64;
    for i in 2..=limit {
        if num % i == 0 {
            return false;
        }
    }
    true
}

// Function to count primes and Buboliceus primes in a range
fn count_primes(range: RangeInclusive<u64>) -> (usize, usize) {
    let mut prime_count = 0;
    let mut buboliceus_count = 0;
    for num in range {
        if is_prime(num) {
            prime_count += 1;
            // Check if hexadecimal representation ends with 'B'
            let hex_str = format!("{:x}", num);
            if hex_str.ends_with('b') || hex_str.ends_with('B') {
                buboliceus_count += 1;
            }
        }
    }
    (prime_count, buboliceus_count)
}

// Actix web handlers
#[get("/is_prime/{number}")]
async fn is_prime_handler(path: web::Path<u64>) -> impl Responder {
    let number = path.into_inner();
    let prime_result = is_prime(number);
    format!("{} is prime: {}", number, prime_result)
}

#[get("/count_primes")]
async fn count_primes_handler(query: web::Query<PrimeQuery>) -> impl Responder {
    let min = query.min.unwrap_or(1);
    let max = query.max.unwrap_or(100000);

    let range = min..=max;
    let (prime_count, buboliceus_count) = count_primes(range);

    format!(
        "Number of primes between {} and {}: {}\nNumber of Buboliceus primes: {}",
        min, max, prime_count, buboliceus_count
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .service(is_prime_handler)
            .service(count_primes_handler)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
