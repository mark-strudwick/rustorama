#[derive(Debug)]
struct Pokemon {
    id: Option<i32>,
    name: String,
    url: String,
    sprite_url: Option<String>,
}

async fn add_pokemon() {
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

    for p in pokemon_list.iter_mut() {
        let pokemon = rustemon::pokemon::pokemon::get_by_name(&p.name, &rustemon_client)
            .await
            .unwrap();
        p.id = Some(pokemon.id as i32);
        p.sprite_url = Some(pokemon.sprites.front_default.unwrap());
    }
}
