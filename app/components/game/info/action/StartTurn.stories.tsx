import React from 'react';
import { Meta, Story } from '@storybook/react';
import { StartTurn, ClueProps } from './StartTurn';

export default {
  title: 'Clue',
  component: StartTurn,
} as Meta;

const Template: Story<ClueProps> = (args) => <StartTurn {...args} />;

export const Default = Template.bind({});
Default.args = {};
