const MAX_VALUE: u32 = 4_000_000;

fn main() {
    let fib = generate_fibonacci_sequence();
    println!("Fibonacci sequence: {:?}", &fib);
    let even_values = get_even_values(fib);
    println!("Fibonacci sequence: {:?}", &even_values);
    let summed_values = summed_values(even_values);
    println!("Summed value of even fibonacci numbers: {}", &summed_values);
}

fn generate_fibonacci_sequence() -> Vec<u32> {
  let mut sequence = Vec::new();
  let (mut previous, mut current) = (0, 1);

  while current <= MAX_VALUE {
    sequence.push(current);
    let next = previous + current;
    previous = current;
    current = next;
  }
  sequence
}

fn get_even_values(fib: Vec<u32>) -> Vec<u32> {
  let mut new_vec: Vec<u32> = Vec::new();

  for i in fib {
    if i % 2 == 0 {
      new_vec.push(i);
    }
  }
  new_vec
}

fn summed_values(even_numbers: Vec<u32>) -> u32 {
  let mut sum: u32 = 0;
  
  for i in even_numbers {
    sum += i;
  }
  sum
}