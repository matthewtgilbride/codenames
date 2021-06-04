export type Team = 'Blue' | 'Red';

export interface Player {
  team: Team;
  is_spy_master: boolean;
  name: string;
}

export type CardType = 'Neutral' | 'Death' | 'Blue' | 'Red';

export interface CardColor {
  color: CardType | null;
  word: string;
}

export interface GameState {
  board: CardColor[];
  name: string;
  first_team: Team;
  turn: Team;
  players: { [key: string]: Player };
  guesses: number[];
}
