import React from 'react';
import ReactDOM from 'react-dom';
import { injectTheme } from '~/styles';
import App from './App';
import './index.css';

injectTheme();

ReactDOM.render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
  document.getElementById('root')
);
