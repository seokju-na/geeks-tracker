import { styled } from './styles';

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
