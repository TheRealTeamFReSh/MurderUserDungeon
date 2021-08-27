#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    MainGame,
    ConsoleOpenedState,
    PeepholeOpenedState,
    PlayerSleepingState,
    GameOverState,
    PlayerPeeingState,
    PlayerOrderingPizzaState,
    PlayerEatingState,
    MainMenu,
    ControlMenu,
}
