import React from 'react';
import { Meta, Story } from '@storybook/react';
import { action } from '@storybook/addon-actions';
import { Game, GameProps } from './Game';
import { defaultGame } from '../../stories/testData';

export default {
  title: 'Game',
  component: Game,
} as Meta;

const Template: Story<GameProps> = (args) => <Game {...args} />;

export const EmptyGame = Template.bind({});
EmptyGame.args = {
  API_URL: '',
  game: defaultGame,
  onGuess: () => action('onGuess'),
};
