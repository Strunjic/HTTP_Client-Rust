use reqwest::blocking::{Client, ClientBuilder}; 
use reqwest::redirect::Policy;

fn main() {
    let http_client =  Client::new();
    let http_result = http_client.get("https://trevorsullivan.net/").send();

    if http_result.is_ok() {
        println!("Body: {:#?}", http_result.ok().unwrap().text().unwrap());
    } else if http_result.is_err() {
        println!("Error: {:#?}", http_result.err());
    }

    let post_result = http_client.post("http://localhost:3000/send_data")
        .body("{\"first_name\":\"Rale\"")
        .header("User-Agent", "Rale Rust Http Client").send();

    if post_result.is_ok() {
        println!("{:#?}", post_result.ok().unwrap().text().unwrap());
    } else if post_result.is_err() {
        println!("Error: {:#?}", post_result.err());
    }

    //Exaple for Redirects

    let redirect_policy = Policy::limited(0);
    let http_client = ClientBuilder::new().redirect(redirect_policy).build().ok().unwrap();
    let http_result = http_client.get("http://localhost:3000/weather").send();
    if http_result.is_ok() {
        println!("{:#?}", http_result.ok().unwrap().text().unwrap());
    } else if http_result.is_err() {
        println!("{:#?}", http_result);
    }
}
