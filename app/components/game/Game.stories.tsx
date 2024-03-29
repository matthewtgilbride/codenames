import React from 'react';
import { Meta, Story } from '@storybook/react';
import { Game, GameProps } from './Game';

export default {
  title: 'Game',
  component: Game,
} as Meta;

const Template: Story<GameProps> = (args) => <Game {...args} />;

export const EmptyGame = Template.bind({});
EmptyGame.args = {};
