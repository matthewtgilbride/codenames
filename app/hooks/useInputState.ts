import { ChangeEventHandler, useState } from 'react';

export const useInputState = (): [
  string,
  ChangeEventHandler<HTMLInputElement>,
] => {
  const [value, setValue] = useState('');
  const onChange: ChangeEventHandler<HTMLInputElement> = (e) =>
    setValue(e.target.value);
  return [value, onChange];
};
