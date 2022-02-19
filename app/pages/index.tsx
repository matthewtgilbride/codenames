import { Route, Routes } from 'react-router-dom';
import { GamePlayerContainer } from '../components/game/GamePlayer';
import { GameLandingContainer } from '../components/game/GameLanding';
import { GameListContainer } from '../components/game/GameList';
import { Home } from '../components/Home';

const App = () => (
  <Routes>
    <Route path="/game/:name/:player" element={<GamePlayerContainer />} />
    <Route path="/game/:name" element={<GameLandingContainer />} />
    <Route path="/game" element={<GameListContainer />} />
    <Route path="/" element={<Home />} />
  </Routes>
);

export default App;
