
pub fn square<T>(x: T) -> T 
where T: std::ops::Mul<Output=T> + Copy
{  
    x*x
}