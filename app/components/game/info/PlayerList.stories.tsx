import React from 'react';
import { Meta, Story } from '@storybook/react';
import { PlayerList, PlayerListProps } from './PlayerList';

export default {
  title: 'PlayerList',
  component: PlayerList,
} as Meta;

const Template: Story<PlayerListProps> = (args) => <PlayerList {...args} />;

export const Empty = Template.bind({});
Empty.args = {
  team: 'Blue',
  spyMaster: false,
};

export const NonEmpty = Template.bind({});
NonEmpty.args = {
  team: 'Blue',
  spyMaster: false,
};

export const CurrentPlayer = Template.bind({});
CurrentPlayer.args = {
  playerName: 'Matt',
  team: 'Blue',
  spyMaster: false,
};
