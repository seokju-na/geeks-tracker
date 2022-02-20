import { styled } from '@stitches/react';

export default function App() {
  return (
    <main>
      <Button>Hello</Button>
    </main>
  );
}

const Button = styled('button', {
  height: 100,
});
