use reqwest;

fn main() {
    let url="https://memos.moyuin.top/api/v1/memos";
    println!("正在调用API :{}",url);
    let response=reqwest::blocking::get(url).expect("请求失败").text().expect("无法获取内容");
    println!("\n获取内容\n");
    println!("{}",response);
}
