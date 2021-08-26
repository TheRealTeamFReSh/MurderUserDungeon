#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    MainGame,
    ConsoleOpenedState,
    PlayerSleepingState,
    GameOverState(bool), // bool indicates whether to hide the player's sprite
}