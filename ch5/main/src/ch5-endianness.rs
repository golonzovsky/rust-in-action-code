use std::mem::transmute;

fn main() {
  let big_endian: [u8; 4] = [0xAA, 0xBB, 0xCC, 0xDD];
  let little_endian: [u8; 4] = [0xDD, 0xCC, 0xBB, 0xAA];

  let a: i32 = unsafe { transmute(big_endian) };
  let b: i32 = unsafe { transmute(little_endian) };

  println!("{} vs {}", a, b);

  let pr_a: u32 = unsafe { std::mem::transmute(a) };
  let pr_b: u32 = unsafe { std::mem::transmute(b) };

  println!("{:032b}", pr_a);
  println!("{:032b}", pr_b);
}
