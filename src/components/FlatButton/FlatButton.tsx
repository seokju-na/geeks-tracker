import { ComponentProps } from '@stitches/react';
import { ElementRef, forwardRef } from 'react';
import { styled } from '../../styles';

const Button = styled('button', {
  all: 'unset',
  display: 'inline-flex',
  alignItems: 'center',
  justifyContent: 'center',
  height: 28,
  padding: '0 $md',
  color: '$text',
  fontSize: '$md',
  '&:focus': {
    backgroundColor: '$blue600',
  },
});

interface Props extends ComponentProps<typeof Button> {}

export const FlatButton = forwardRef<ElementRef<'button'>, Props>((props, ref) => {
  return <Button ref={ref} {...props} />;
});
