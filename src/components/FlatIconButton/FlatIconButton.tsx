import { ComponentProps } from '@stitches/react';
import { ElementRef, forwardRef, ReactElement } from 'react';
import { FlatButton } from '../FlatButton';

type Ref = ElementRef<typeof FlatButton>;

interface Props extends ComponentProps<typeof FlatButton> {
  'aria-label': string;
  children: ReactElement;
}

export const FlatIconButton = forwardRef<Ref, Props>((props, ref) => {
  return (
    <FlatButton
      ref={ref}
      css={{
        color: '$icon',
      }}
      {...props}
    />
  );
});
