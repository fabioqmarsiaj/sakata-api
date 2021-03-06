table! {
    base_cards (id) {
        id -> Nullable<Integer>,
        name -> Varchar,
        overall_power -> Tinyint,
        class -> Tinyint,
        genre -> Tinyint,
        mal_id -> Integer,
    }
}

table! {
    player_cards (id) {
        id -> Nullable<Integer>,
        base_card_id -> Integer,
        player_id -> Integer,
        rarity -> Tinyint,
        quantity -> Tinyint,
    }
}

table! {
    players (id) {
        id -> Nullable<Integer>,
        nickname -> Varchar,
        coins -> Smallint,
        stardust -> Smallint,
        discord_id -> Bigint,
    }
}

joinable!(player_cards -> base_cards (base_card_id));
joinable!(player_cards -> players (player_id));

allow_tables_to_appear_in_same_query!(
    base_cards,
    player_cards,
    players,
);
