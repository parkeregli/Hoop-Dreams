/**
 * TypeScript types matching the Rust backend game state structures.
 * These types provide type safety for frontend-backend communication.
 */

/**
 * All possible player actions during gameplay.
 */
export type PlayerAction =
  | "Pass"
  | "Drive"
  | "Rebound"
  | "Layup"
  | "Dunk"
  | "ShootOfDribble"
  | "Shoot"
  | "SpotUp"
  | "Cut"
  | "BallScreen"
  | "OffBallScreen"
  | "Contest"
  | "Block"
  | "DefendTight"
  | "Defend"
  | "DefendLoose"
  | "Steal"
  | "Foul"
  | "Idle";

/**
 * All possible court areas where a player can be positioned.
 */
export type CourtArea =
  | "Basket"
  | "RestrictedAreaLeft"
  | "RestrictedAreaMiddle"
  | "RestrictedAreaRight"
  | "LowPostLeft"
  | "LowPostRight"
  | "ShortCornerLeft"
  | "ShortCornerRight"
  | "ElbowLeft"
  | "ElbowRight"
  | "FreeThrowLine"
  | "MidrangeBaselineLeft"
  | "MidrangeBaselineRight"
  | "MidrangeWingLeft"
  | "MidrangeCenter"
  | "MidrangeWingRight"
  | "ThreePointLineCornerLeft"
  | "ThreePointLineCornerRight"
  | "ThreePointLineWingLeft"
  | "ThreePointLineWingRight"
  | "ThreePointLineCenter"
  | "Center"
  | "Backcourt"
  | "SidelineLeft"
  | "SidelineRight"
  | "BaselineLeft"
  | "BaselineRight"
  | "OutOfBounds";

/**
 * Which team has possession.
 */
export type Possession = "Home" | "Away";

/**
 * Player's current state during the game.
 */
export interface PlayerState {
  action: PlayerAction;
  current_area: CourtArea;
}

/**
 * Player attributes for skills and physical stats.
 */
export interface PlayerAttributes {
  inside: number;
  mid: number;
  three: number;
  ft: number;
  layup: number;
  dunk: number;
  spd: number;
  quickness: number;
  vertical: number;
  strength: number;
  stamina: number;
  hustle: number;
  bball_iq: number;
  passing: number;
  ball_handle: number;
  post_moves: number;
  def_rebound: number;
  off_rebound: number;
  shot_contest: number;
  block: number;
  steal: number;
  interior_def: number;
  perimeter_def: number;
  ath: number;
}

/**
 * Basic player information.
 */
export interface Player {
  id: number;
  first_name: string;
  last_name: string;
  position: string;
  age: number;
  height: number;
  weight: number;
  attr: PlayerAttributes;
}

/**
 * Player combined with their current game state.
 */
export interface PlayerWithState {
  player: Player;
  state: PlayerState;
}

/**
 * Team information.
 */
export interface Team {
  id: number;
  name: string;
  city: string;
}

/**
 * Current state of a team during the game.
 */
export interface TeamState {
  active_players: [Player, PlayerState][];
  bench: [Player[], Player[]];
}

/**
 * The main game state structure.
 */
export interface GameState {
  time: { secs: number; nanos: number };
  shot_clock: { secs: number; nanos: number };
  period: number;
  possession: [Possession, number] | null;
  team_state: [TeamState, TeamState];
  fouls: [number, number];
  timeouts: [number, number];
  score: [number, number];
}

/**
 * A game event that occurred during simulation.
 */
export interface GameEvent {
  action: string;
  time: string;
  period: number;
  possession: Possession | null;
}

/**
 * The full game structure returned from the backend.
 */
export interface Game {
  teams: [Team, Team];
  events: GameEvent[];
  state: GameState;
  sim: boolean;
}

/**
 * Player data as received from the player_states event.
 * The backend sends an array of [Player, PlayerState] tuples.
 */
export type PlayerStatesPayload = [Player, PlayerState][];

/**
 * Score data as received from the game_score event.
 */
export type GameScorePayload = [number, number];

/**
 * Clock data as received from the game_clock event.
 * Tuple of [time, shot_clock, period, possession]
 */
export type GameClockPayload = [
  { secs: number; nanos: number },
  { secs: number; nanos: number },
  number,
  [Possession, number] | null
];
