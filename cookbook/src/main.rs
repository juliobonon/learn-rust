use rand::Rng;

fn shadowing() {
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);
}

fn randomnumber(){
    let mut rng = rand::thread_rng();

    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();
    println!("Random u8: {}", n1);
    println!("Random u16: {}", n2);
    println!("Random u32: {}", rng.gen::<u32>());
    println!("Random i32: {}", rng.gen::<i32>());
    println!("Random float: {}", rng.gen::<f64>());
}

fn randompasswords(phrase: &str) {
  let charset: &[u8] = phrase.as_bytes();

  const PASSWORD_LEN: usize = 30;
  let mut rng = rand::thread_rng();

  let password: String = (0..PASSWORD_LEN)
      .map(|_| {
          let idx = rng.gen_range(0..charset.len());
          charset[idx] as char
      })
      .collect();

    println!("{:?}", password);
}

fn main() {
    randomnumber();
    randompasswords("hickaohickao");
    shadowing();
}
