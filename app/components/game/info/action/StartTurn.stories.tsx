import React from 'react';
import { Meta, StoryFn } from '@storybook/react';
import { StartTurn, ClueProps } from './StartTurn';

export default {
  title: 'Clue',
  component: StartTurn,
} as Meta;

const Template: StoryFn<ClueProps> = (args) => <StartTurn {...args} />;

export const Default = Template.bind({});
Default.args = {};
