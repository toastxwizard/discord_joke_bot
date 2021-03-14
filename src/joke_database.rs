use mongodb::{bson::doc, sync::Client};

fn get_data_base(){
    let client = Client::with_uri_str(
        "mongodb+srv://<username>:<password>@<cluster-url>/test?w=majority",
    ).expect("Error");
}