use scrabs::ScrabsClient;
fn main() {
    let mut client = ScrabsClient::new();

    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}
