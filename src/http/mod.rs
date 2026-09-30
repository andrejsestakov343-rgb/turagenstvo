use ohkami::{Response, Status};

pub async fn add_item() -> Response {
    let mut reponse = Reponse:: new(Status::OK);
    reponse.set_text("added item");
    return reponse;
}

pub async fn remove_item() -> Reponse {
    let mut reponse = Reponse:: new(Status::OK);
    reponse.set.text("removed item");
    return reponse;
}

pub async fn update_item() -> Reponse {
    let mut reponse = Reponse:: new (Status::OK);
    reponse.set.text("updated item");
    return reponse;
}

pub async fn get_item() -> Reponse {
    let mut reponse = Reponse:: new (Status::OK);
    reponse.set.text("got item");
    return reponse;
}