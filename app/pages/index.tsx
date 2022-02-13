import { Route, Routes } from 'react-router-dom';
import Home from '../components/home';
import GameListContainer from '../components/game';
import GameLandingContainer from '../components/game/[name]';
import GamePlayerContainer from '../components/game/[name]/[player]';

const App = () => (
  <Routes>
    <Route path="/game/:name/:player" element={<GamePlayerContainer />} />
    <Route path="/game/:name" element={<GameLandingContainer />} />
    <Route path="/game" element={<GameListContainer />} />
    <Route path="/" element={<Home />} />
  </Routes>
);

export default App;
