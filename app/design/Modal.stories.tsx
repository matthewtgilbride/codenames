import React from 'react';
import { Meta, StoryFn } from '@storybook/react';
import { Modal, ModalProps } from './Modal';

export default {
  title: 'Modal',
  component: Modal,
} as Meta;

const Template: StoryFn<ModalProps & { contentHeight?: number }> = (args) => (
  <div style={{ height: args.contentHeight ?? '100%' }}>
    <Modal {...args} />
  </div>
);

export const Default = Template.bind({});
Default.args = {
  isOpen: false,
};

export const Tall = Template.bind({});
Tall.args = {
  isOpen: false,
  contentHeight: 1000,
};
