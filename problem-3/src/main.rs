const TARGET_VALUE: i64 = 600_851_475_143;

fn main() {
    match find_largest_prime(TARGET_VALUE) {
      Some(prime) => println!("The largest prime number less than or equal to {} is: {}", TARGET_VALUE, prime),
      None => println!("There is no prime number less than or equal to {}", TARGET_VALUE),
  }
}

fn find_largest_prime(num: i64) -> Option<i64> {
  if num < 2 {
    return None
  }

  for num in (2..=num).rev() {
    if is_prime(num) {
      return Some(num);
    }
  }

  None
}

fn is_prime(n: i64) -> bool {
  if n <= 1 {
    return false;
  }

  let limit = (n as f64).sqrt() as i64;

  for i in 2..=limit {
    if n % i == 0 {
      return false
    }
  }

  true
}