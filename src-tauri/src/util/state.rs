use std::sync::{atomic::AtomicBool, Arc, Mutex, PoisonError};

use rusqlite::Connection;
use tauri::{AppHandle, Manager, State};

use crate::game::Game;

#[derive(Debug)]
pub enum StateError {
    LockPoisoned(String),
    NotInitialized(String),
}

impl std::fmt::Display for StateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StateError::LockPoisoned(msg) => write!(f, "Lock poisoned: {}", msg),
            StateError::NotInitialized(msg) => write!(f, "Not initialized: {}", msg),
        }
    }
}

impl std::error::Error for StateError {}

impl<T> From<PoisonError<T>> for StateError {
    fn from(err: PoisonError<T>) -> Self {
        StateError::LockPoisoned(err.to_string())
    }
}

pub struct AppState {
    pub db: std::sync::Mutex<Option<Connection>>,
    pub game: std::sync::Mutex<Option<Game>>,
    pub running: Arc<AtomicBool>,
    pub sim_thread: Mutex<Option<std::thread::JoinHandle<()>>>,
}

pub trait ServiceAccess {
    fn db<F, TResult>(&self, operation: F) -> Result<TResult, StateError>
    where
        F: FnOnce(&Connection) -> TResult;

    fn db_mut<F, TResult>(&self, operation: F) -> Result<TResult, StateError>
    where
        F: FnOnce(&mut Connection) -> TResult;

    fn game<F, TResult>(&self, operation: F) -> Result<TResult, StateError>
    where
        F: FnOnce(&Game) -> TResult;

    fn game_mut<F, TResult>(&self, operation: F) -> Result<TResult, StateError>
    where
        F: FnOnce(&mut Game) -> TResult;
}

impl ServiceAccess for AppHandle {
    fn db<F, TResult>(&self, operation: F) -> Result<TResult, StateError>
    where
        F: FnOnce(&Connection) -> TResult,
    {
        let app_state: State<AppState> = self.state();
        let db_connection_guard = app_state.db.lock()?;
        let db = db_connection_guard.as_ref().ok_or_else(|| {
            StateError::NotInitialized("Database connection not initialized".to_string())
        })?;

        Ok(operation(db))
    }

    fn db_mut<F, TResult>(&self, operation: F) -> Result<TResult, StateError>
    where
        F: FnOnce(&mut Connection) -> TResult,
    {
        let app_state: State<AppState> = self.state();
        let mut db_connection_guard = app_state.db.lock()?;
        let db = db_connection_guard.as_mut().ok_or_else(|| {
            StateError::NotInitialized("Database connection not initialized".to_string())
        })?;

        Ok(operation(db))
    }

    fn game<F, TResult>(&self, operation: F) -> Result<TResult, StateError>
    where
        F: FnOnce(&Game) -> TResult,
    {
        let app_state: State<AppState> = self.state();
        let game_guard = app_state.game.lock()?;
        let game = game_guard
            .as_ref()
            .ok_or_else(|| StateError::NotInitialized("Game not initialized".to_string()))?;

        Ok(operation(game))
    }

    fn game_mut<F, TResult>(&self, operation: F) -> Result<TResult, StateError>
    where
        F: FnOnce(&mut Game) -> TResult,
    {
        let app_state: State<AppState> = self.state();
        let mut game_guard = app_state.game.lock()?;
        let game = game_guard
            .as_mut()
            .ok_or_else(|| StateError::NotInitialized("Game not initialized".to_string()))?;

        Ok(operation(game))
    }
}
