import React from 'react';
import ReactDOM from 'react-dom';
import App from './App';
import './index.css';
import { injectTheme } from './styles';

injectTheme();

ReactDOM.render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
  document.getElementById('root')
);
