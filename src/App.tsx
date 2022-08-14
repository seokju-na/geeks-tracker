import { Router } from '@tanstack/react-location';
import { useGlobalEscKeydown } from './hooks/useGlobalEscKeydown';
import { location } from './location';
import { Main } from './routes/Main';

export default function App() {
  useGlobalEscKeydown();

  return (
    <Router
      location={location}
      routes={[
        {
          path: '/',
          element: <Main />,
        },
      ]}
    />
  );
}
