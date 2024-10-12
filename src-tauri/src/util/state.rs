use std::sync::{atomic::AtomicBool, Arc, Mutex};

use rusqlite::Connection;
use tauri::{AppHandle, Manager, State};

use crate::game::Game;

pub struct AppState {
    pub db: std::sync::Mutex<Option<Connection>>,
    pub game: std::sync::Mutex<Option<Game>>,
    pub running: Arc<AtomicBool>,
    pub sim_thread: Mutex<Option<std::thread::JoinHandle<()>>>,
}

pub trait ServiceAccess {
    fn db<F, TResult>(&self, operation: F) -> TResult
    where
        F: FnOnce(&Connection) -> TResult;

    fn db_mut<F, TResult>(&self, operation: F) -> TResult
    where
        F: FnOnce(&mut Connection) -> TResult;
    fn game<F, TResult>(&self, operation: F) -> TResult
    where
        F: FnOnce(&Game) -> TResult;
    fn game_mut<F, TResult>(&self, operation: F) -> TResult
    where
        F: FnOnce(&mut Game) -> TResult;
}

impl ServiceAccess for AppHandle {
    fn db<F, TResult>(&self, operation: F) -> TResult
    where
        F: FnOnce(&Connection) -> TResult,
    {
        let app_state: State<AppState> = self.state();
        let db_connection_guard = app_state.db.lock().unwrap();
        let db = db_connection_guard.as_ref().unwrap();

        operation(db)
    }

    fn db_mut<F, TResult>(&self, operation: F) -> TResult
    where
        F: FnOnce(&mut Connection) -> TResult,
    {
        let app_state: State<AppState> = self.state();
        let mut db_connection_guard = app_state.db.lock().unwrap();
        let db = db_connection_guard.as_mut().unwrap();

        operation(db)
    }

    fn game<F, TResult>(&self, operation: F) -> TResult
    where
        F: FnOnce(&Game) -> TResult,
    {
        let app_state: State<AppState> = self.state();
        let game_guard = app_state.game.lock().unwrap();
        let game = game_guard.as_ref().unwrap();
        operation(game)
    }

    fn game_mut<F, TResult>(&self, operation: F) -> TResult
    where
        F: FnOnce(&mut Game) -> TResult,
    {
        let app_state: State<AppState> = self.state();
        let mut game_guard = app_state.game.lock().unwrap();
        let game = game_guard.as_mut().unwrap();
        operation(game)
    }
}
