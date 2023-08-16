pub trait ReplayData {
    fn get_events()
    {}

    fn get_player_count() -> u32 { 0 }

    fn get_replay_stats() {}

    fn get_game_total_frame(replay_index: usize)
    {}

    fn get_username() {}

    fn get_replay_count()
    {}
}
