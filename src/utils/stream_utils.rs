use std::io::BufReader;

pub fn xor(s1: impl Iterator<Item=u8>, s2: impl Iterator<Item=u8>) -> impl Iterator<Item=u8> {
  let zip = s1.zip(s2);
  let xor = zip.map(|(x, y)| x ^ y);
  xor
}