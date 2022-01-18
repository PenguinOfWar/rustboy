import React, { useEffect } from 'react';

import * as rustboy from 'rustboy';

import logo from '../../logo.svg';
import './App.css';

function App() {
  useEffect(() => {
    if (rustboy.boot && rustboy.boot instanceof Function) {
      rustboy.boot();
    }
    if (rustboy.list_games && rustboy.list_games instanceof Function) {
      const games = rustboy.list_games();
      console.log(games);
    }
  }, []);

  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          Edit <code>src/App.tsx</code> and save to reload.
        </p>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>
      </header>
    </div>
  );
}

export default App;
