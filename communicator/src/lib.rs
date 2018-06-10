pub mod client;
pub mod network;

#[cfg(test)]
mod tests {
    use super::client;
    use super::client::*;
    use super::client::connect;

    #[test]
    fn it_works() {
        ::client::connect(); // to the root!
        super::client::connect(); // one up!
        client::connect();
        connect();
    }
}
