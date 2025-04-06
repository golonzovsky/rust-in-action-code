use colored::*;

fn main() {
  let n: f32 = -2342.2342344; //-42.42;
  println!();
  println!("num: {:.10}", n);

  let (sign, exponent, mantissa) = to_parts(n);
  println!();
  println!("{}     {:032b}", "full:".green().bold(), n.to_bits());
  println!("{}     {:b}", "sign:".green().bold(), sign);
  println!("{}  {:08b}", "exponent:".green().bold(), exponent);
  println!("{}          {:022b}", "mantissa:".green().bold(), mantissa);

  let (sign, exponent, mantissa) = decode(sign, exponent, mantissa);
  println!();
  println!(
    "{:.}={}*{}*2^{}={:.10}",
    n,
    sign,
    mantissa,
    exponent.log2(),
    sign * mantissa * exponent
  );
}

fn to_parts(n: f32) -> (u32, u32, u32) {
  let bits = n.to_bits();
  let sign = bits >> 31;
  let exponent = (bits >> 23) & 0xff; // getting rid of sign
  let mantissa = bits & 0x7fffff; // 22
  (sign, exponent, mantissa)
}

const BIAS: i32 = 127;

fn decode(sign: u32, exponent: u32, fraction: u32) -> (f32, f32, f32) {
  let signed_1 = (-1.0_f32).powf(sign as f32);
  let exponent_power = (exponent as i32 - BIAS) as f32;
  let exponent = 2_f32.powf(exponent_power);
  let mut mantissa: f32 = 1.0;

  for i in 0..23 {
    let mask = 1 << i;
    if fraction & mask == 0 {
      continue;
    }
    let i_ = i as f32;
    let weight = 2_f32.powf(i_ - 23.0);
    mantissa += weight;
  }
  (signed_1, exponent, mantissa)
}
