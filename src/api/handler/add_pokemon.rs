use aws_sdk_dynamodb::{
    error::PutItemError, model::AttributeValue, output::PutItemOutput, types::SdkError,
};
use axum::{extract::State, response::Json};
use serde_json::{json, Value};

#[derive(Debug)]
pub struct Pokemon {
    id: Option<i32>,
    name: String,
    url: String,
    sprite_url: Option<String>,
}

pub async fn add_pokemon(
    State(client): State<aws_sdk_dynamodb::Client>,
) -> Result<Json<Value>, ()> {
    println!("Fetching pokemon data");
    let rustemon_client = rustemon::client::RustemonClient::default();
    let pokemons = rustemon::pokemon::pokemon::get_all_pages(&rustemon_client)
        .await
        .unwrap();
    let mut pokemon_list: Vec<Pokemon> = pokemons
        .into_iter()
        .map(|p| Pokemon {
            id: None,
            name: p.name,
            url: p.url,
            sprite_url: None,
        })
        .collect();

    pokemon_list.drain(494..); // There's probs a better way to do this

    println!("Fetching pokemon sprites and adding to database");

    for p in pokemon_list.iter_mut() {
        let pokemon = rustemon::pokemon::pokemon::get_by_name(&p.name, &rustemon_client)
            .await
            .unwrap();
        p.id = Some(pokemon.id as i32);
        p.sprite_url = Some(pokemon.sprites.front_default.unwrap());

        let attributes = [
            ("pk", AttributeValue::S(p.id.unwrap().to_string())),
            ("name", AttributeValue::S(p.name.to_string())),
            ("url", AttributeValue::S(p.url.to_string())),
            (
                "sprite_url",
                AttributeValue::S(p.sprite_url.clone().unwrap().to_string()),
            ),
        ];

        let mut r = client.put_item().table_name("pokemon");
        for (k, v) in attributes {
            r = r.item(k, v);
        }
        r.send().await.unwrap();
    }

    Ok(Json(json!({
        "message": "Successfully added pokemon to database"
    })))
}
