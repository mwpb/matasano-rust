use crate::utils::stream_utils::xor;

#[cfg(test)]
fn s1c2() -> Result<String, hex::FromHexError> {
  let hexes1 = String::from("1c0111001f010100061a024b53535009181c");
  let bytes1 = hex::decode(hexes1)?;
  let hexes2 = String::from("686974207468652062756c6c277320657965");
  let bytes2 = hex::decode(hexes2)?;

  let xor = xor(
    bytes1.iter().cloned(),
    bytes2.iter().cloned(),
  );

  let coll = xor.collect::<Vec<u8>>();
  Ok(hex::encode(coll))
}

#[cfg(test)]
mod s1c2_tests {
  use super::*;

  #[test]
  fn test_answer() {
    let hex_string = match s1c2() {
      Err(e) => panic!(e),
      Ok(s) => s,
    };
    assert_eq!(
      hex_string,
      "746865206b696420646f6e277420706c6179"
    );
  }
}