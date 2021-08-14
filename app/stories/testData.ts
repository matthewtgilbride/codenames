import { GameState } from '../model';

export const defaultGame: GameState = {
  name: 'Pie-Poison',
  players: {
    operative: {
      team: 'Blue',
      name: 'Operative',
      spymaster_secret: null,
    },
    'spy master': {
      team: 'Blue',
      name: 'Spy Master',
      spymaster_secret: 'foo',
    },
  },
  turns: [
    {
      type: 'Started',
      data: {
        spymaster: {
          team: 'Blue',
          name: 'Spy Master',
          spymaster_secret: 'foo',
        },
        clue: ['Foo', 1],
        guesses: [
          [
            {
              team: 'Blue',
              name: 'Operative',
              spymaster_secret: null,
            },
            0,
          ],
        ],
      },
    },
  ],
  board: [
    {
      color: 'Red',
      word: 'Glass',
    },
    {
      color: null,
      word: 'Princess',
    },
    {
      color: null,
      word: 'Laser',
    },
    {
      color: null,
      word: 'Paper',
    },
    {
      color: null,
      word: 'Hotel',
    },
    {
      color: null,
      word: 'Egypt',
    },
    {
      color: null,
      word: 'Hospital',
    },
    {
      color: null,
      word: 'Stock',
    },
    {
      color: null,
      word: 'Tap',
    },
    {
      color: null,
      word: 'Mouth',
    },
    {
      color: null,
      word: 'Poison',
    },
    {
      color: null,
      word: 'Revolution',
    },
    {
      color: null,
      word: 'Face',
    },
    {
      color: null,
      word: 'Tag',
    },
    {
      color: null,
      word: 'Root',
    },
    {
      color: null,
      word: 'Whale',
    },
    {
      color: null,
      word: 'Row',
    },
    {
      color: null,
      word: 'Bridge',
    },
    {
      color: null,
      word: 'Plastic',
    },
    {
      color: null,
      word: 'Fan',
    },
    {
      color: null,
      word: 'Theater',
    },
    {
      color: null,
      word: 'Kid',
    },
    {
      color: null,
      word: 'Rome',
    },
    {
      color: null,
      word: 'Dinosaur',
    },
    {
      color: null,
      word: 'Chair',
    },
  ],
};
