use ohkami::{Ohkami, Route};

mod contracts;
mod contracts_impl;
mod models;
mod http;

fn main() -> smol::io::Result<()> {
    smol::block_on(async {
        Ohkami::new((
            "/add-item".POST(http::add_item),
            "/remove-item".DELETE(http::remove_item),
            "/update-item".PUT(http::update_item),
            "/get-item".GET(http::get_item),
        ))
        .howl("localhost:3000")
        .await;
        Ok(())
    })
}
