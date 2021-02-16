export type Team = 'Blue' | 'Red';

export interface Player {
  team: Team;
  is_spy_master: boolean;
  name: string;
}

export type CardType = 'Neutral' | 'Death' | 'Blue' | 'Red';

export interface CardColor {
  color?: CardType;
  word: string;
}

export interface GameState {
  board: CardColor[];
  name: string;
  turn: Team;
  players: { [key: string]: Player };
  guesses: number[];
}
