import React from 'react';
import { Meta, Story } from '@storybook/react';
import { Clue, ClueProps } from '../components/game/info/Clue';

export default {
  title: 'GiveClue',
  component: Clue,
} as Meta;

const Template: Story<ClueProps> = (args) => <Clue {...args} />;

export const Default = Template.bind({});
Default.args = {
  isOpen: false,
};
