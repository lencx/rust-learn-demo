#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

/*
[root]
|- [src]
|   |- client.rs
|   |- lib.rs
|   `- [network]
|         |- mod.rs
|         `- server.rs
*/
mod client;

mod network;
