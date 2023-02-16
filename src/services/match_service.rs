use diesel::{RunQueryDsl, PgConnection};
use crate::models::match_models as MatchModels;

pub mod match_service {}

pub fn add_match(conn: &mut PgConnection, new_match: MatchModels::NewMatchDto){
    use crate::schema::matches;

    let new_match = MatchModels::NewMatch {
        title: new_match.title,
        body: new_match.body,
        player_one_name: new_match.player_one_name,
        player_two_name: new_match.player_two_name
    };

    diesel::insert_into(matches::table)
        .values(&new_match)
        .get_result(conn)
        .expect("Error saving new post")
}