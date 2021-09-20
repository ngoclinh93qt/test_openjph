
use image::*;
fn main() {
    let img = image::open("./tests/lex.jpg").unwrap();
    let width = img.width();
    let dimentsion = img.dimensions();

    let img8 = img.into_rgb8();
    let mut data = img8.into_raw() as Vec<u8>;

    let out  = openjphffi::htj2k_encode_fullquality(data , dimentsion.0 as usize, dimentsion.1 as usize );
    //let elapase = now.elapsed();
    println!("{}",out.len());  
    

}
