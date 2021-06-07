import React from 'react';
import { Meta, Story } from '@storybook/react';
import { action } from '@storybook/addon-actions';
import { PlayerList, PlayerListProps } from '../components/game/PlayerList';

export default {
  title: 'PlayerList',
  component: PlayerList,
} as Meta;

const Template: Story<PlayerListProps> = (args) => <PlayerList {...args} />;

export const Empty = Template.bind({});
Empty.args = {
  onJoin: action('join'),
  players: {},
  team: 'Blue',
  isSpyMaster: false,
};

export const NonEmpty = Template.bind({});
NonEmpty.args = {
  onJoin: action('join'),
  players: {
    Matt: { name: 'Matt', is_spy_master: false, team: 'Blue' },
    Jamie: { name: 'Jamie', is_spy_master: false, team: 'Blue' },
  },
  team: 'Blue',
  isSpyMaster: false,
};

export const CurrentPlayer = Template.bind({});
CurrentPlayer.args = {
  onJoin: action('join'),
  players: {
    Matt: { name: 'Matt', is_spy_master: false, team: 'Blue' },
    Jamie: { name: 'Jamie', is_spy_master: false, team: 'Blue' },
  },
  currentPlayer: 'Matt',
  team: 'Blue',
  isSpyMaster: false,
};
