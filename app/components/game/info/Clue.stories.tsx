import React from 'react';
import { Meta, Story } from '@storybook/react';
import { action } from '@storybook/addon-actions';
import { Clue, ClueProps } from './Clue';

export default {
  title: 'Clue',
  component: Clue,
} as Meta;

const Template: Story<ClueProps> = (args) => <Clue {...args} />;

export const Default = Template.bind({});
Default.args = {
  isOpen: false,
  onRequestClose: action('onRequestClose'),
};
