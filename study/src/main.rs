

fn greet(){
    let _zh_tw="你好，世界！";
    let zh="你好，世界！";
    let en="Hello,World!";
    let fr ="Bonjour,le monde!";
    let regions=[fr,en,zh];
    //for region in regions.iter(){
    for region in regions{
        println!("{}",region);
    }
}
fn main() {
    greet();
}
