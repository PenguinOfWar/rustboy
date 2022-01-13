import React, { useEffect } from 'react';

import * as rustboy from 'rustboy';

import logo from '../../logo.svg';
import './App.css';

function App() {
  useEffect(() => {
    console.log(rustboy);
    if (rustboy.greet && rustboy.greet instanceof Function) {
      rustboy.greet();
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
