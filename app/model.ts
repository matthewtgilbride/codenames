export type Team = 'Blue' | 'Red';

export interface Player {
  team: Team;
  spymaster_secret: string | null;
  name: string;
}

export type CardType = 'Neutral' | 'Death' | 'Blue' | 'Red';

export interface CardColor {
  color: CardType | null;
  word: string;
}

export interface PendingTurn {
  type: 'Pending';
  data: Team;
}

export interface StartedTurn {
  type: 'Started';
  data: {
    spymaster: Player;
    clue: [string, number];
    guesses: [Player, number][];
  };
}

export type Turn = PendingTurn | StartedTurn;

export interface GameState {
  name: string;
  players: { [key: string]: Player };
  turns: Turn[];
  board: CardColor[];
}

export const firstTeam = (game: GameState): Team => {
  const firstTurn = game.turns.slice().reverse()[0];
  if (firstTurn.type === 'Pending') return firstTurn.data;
  return firstTurn.data.spymaster.team;
};

export const currentTeam = (game: GameState): Team => {
  const [turn] = game.turns;
  if (turn.type === 'Pending') return turn.data;
  return turn.data.spymaster.team;
};

export const currentTurn = (game: GameState): Turn => game.turns[0];

export const isSpyMaster = (player: Player): boolean =>
  player.spymaster_secret !== null;

export const getGuesses = (game: GameState): number[] =>
  game.turns
    .map((t) => {
      if (t.type === 'Pending') return [];
      return t.data.guesses.map(([_, index]) => index);
    })
    .flat();
