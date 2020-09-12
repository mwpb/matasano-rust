#[cfg(test)]
fn c11() -> Result<String, hex::FromHexError> {
  let hex_string = String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
  let bytes = hex::decode(hex_string)?;
  return Ok(base64::encode(bytes));
}

#[cfg(test)]
mod c11_tests {
  use super::*;

  #[test]
  fn test_answer() {
    let base64_string = match c11() {
      Err(e) => panic!(e),
      Ok(s) => s,
    };
    assert_eq!(
      base64_string,
      "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
    );
  }

  #[test]
  fn test_easter_egg() {
    let hex_string = String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d");
    let bytes = match hex::decode(hex_string) {
      Err(e) => panic!(e),
      Ok(s) => s,
    };
    assert_eq!(
      String::from_utf8_lossy(&*bytes),
      "I'm killing your brain like a poisonous mushroom"
    );
  }
}
