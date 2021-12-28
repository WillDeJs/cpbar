
fn main() {

    let vector =  vec![24;50];
 
    println!("Executing operation...");
    for _ in cpbar::ProgressBar::new(vector.iter()).with_bounds().with_delims(('|', '|')){
        std::thread::sleep(std::time::Duration::from_millis(200));
    }

}