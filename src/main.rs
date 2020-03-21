
mod kraken;

fn main() {
    let kraken = kraken::Kraken::open().unwrap();
    
    let data = kraken.read().unwrap();
    
    println!("DATA: {:?}", data);

    kraken.set_fan_speed(25).unwrap();

    


}