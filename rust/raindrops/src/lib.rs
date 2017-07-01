pub fn raindrops(num: usize) -> String {
  let mut result: String = String::new();

  if num % 3 == 0 {
      result += "Pling";
  }

  if num % 5 == 0 {
      result += "Plang";
  }

  if num % 7 == 0 {
      result += "Plong";
  }

  if result.is_empty() {
      result += num.to_string().as_str();
  }

  result
}
