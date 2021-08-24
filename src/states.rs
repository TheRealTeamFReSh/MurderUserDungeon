#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    MainGame,
    ConsoleOpenedState,
    ConsoleGame,
}