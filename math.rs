pub fn dist(d1 :Vec<f32>, d2 :Vec<f32>) -> f32
{
    return ((d1[0] - d2[0]).powf(2f32) + (d1[1] - d2[1]).powf(2f32)).sqrt();
}
