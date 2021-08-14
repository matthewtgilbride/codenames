import { GameState } from '../model';

export const defaultGame: GameState = {
  name: 'Duck-Olympus',
  first_team: 'Blue',
  board: [
    { color: null, word: 'Cast' },
    { color: null, word: 'Carrot' },
    {
      color: null,
      word: 'Grass',
    },
    { color: null, word: 'Buffalo' },
    { color: null, word: 'Mouth' },
    {
      color: null,
      word: 'Green',
    },
    { color: null, word: 'Board' },
    { color: null, word: 'Novel' },
    {
      color: null,
      word: 'Root',
    },
    { color: null, word: 'Bill' },
    { color: null, word: 'Penguin' },
    {
      color: null,
      word: 'Stick',
    },
    { color: null, word: 'Wave' },
    { color: null, word: 'Mole' },
    { color: null, word: 'Boot' },
    {
      color: null,
      word: 'Apple',
    },
    { color: null, word: 'Washington' },
    { color: null, word: 'Cycle' },
    {
      color: null,
      word: 'Berlin',
    },
    { color: null, word: 'Ninja' },
    { color: null, word: 'Slug' },
    { color: null, word: 'Fish' },
    {
      color: null,
      word: 'Slip',
    },
    { color: null, word: 'Ruler' },
    { color: null, word: 'Cliff' },
  ],
  turn: 'Red',
  players: {},
  guesses: [],
};
