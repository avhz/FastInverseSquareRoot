fn inv_sqrt(x: f32) -> f32 {
    let i = x.to_bits();
    let i = 0x5f3759df - (i >> 1);
    let y = f32::from_bits(i);

    y * (1.5 - 0.5 * x * y * y)
}

fn main() {}
