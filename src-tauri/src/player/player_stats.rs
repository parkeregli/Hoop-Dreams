pub struct PlayerStats {
    game_id: i64,
    player_id: i64,
    team_id: i64,
    minutes: f64,
    field_goals_made: f64,
    field_goals_attempted: f64,
    field_goal_percentage: f64,
    three_point_field_goals_made: f64,
    three_point_field_goals_attempted: f64,
    three_point_field_goal_percentage: f64,
    free_throws_made: f64,
    free_throws_attempted: f64,
    free_throw_percentage: f64,
    offensive_rebounds: f64,
    defensive_rebounds: f64,
    total_rebounds: f64,
    assists: f64,
    steals: f64,
    blocks: f64,
    turnovers: f64,
    personal_fouls: f64,
    points: f64,
}

impl PlayerStats {
    pub fn new(
        game_id: i64,
        player_id: i64,
        team_id: i64,
        minutes: f64,
        field_goals_made: f64,
        field_goals_attempted: f64,
        three_point_field_goals_made: f64,
        three_point_field_goals_attempted: f64,
        free_throws_made: f64,
        free_throws_attempted: f64,
        offensive_rebounds: f64,
        defensive_rebounds: f64,
        total_rebounds: f64,
        assists: f64,
        steals: f64,
        blocks: f64,
        turnovers: f64,
        personal_fouls: f64,
        points: f64,
    ) -> PlayerStats {
        let field_goal_percentage = field_goals_made / field_goals_attempted;
        let three_point_field_goal_percentage =
            three_point_field_goals_made / three_point_field_goals_attempted;
        let free_throw_percentage = free_throws_made / free_throws_attempted;
        PlayerStats {
            game_id,
            player_id,
            team_id,
            minutes,
            field_goals_made,
            field_goals_attempted,
            field_goal_percentage,
            three_point_field_goals_made,
            three_point_field_goals_attempted,
            three_point_field_goal_percentage,
            free_throws_made,
            free_throws_attempted,
            free_throw_percentage,
            offensive_rebounds,
            defensive_rebounds,
            total_rebounds,
            assists,
            steals,
            blocks,
            turnovers,
            personal_fouls,
            points,
        }
    }
}
