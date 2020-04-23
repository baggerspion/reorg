import App from './assets/layouts/App';
import Conferences from './assets/layouts/Conferences';
import Reviews from './assets/layouts/Reviews';

import React from 'react';
import ReactDOM from 'react-dom';
import { BrowserRouter, Route, Switch } from "react-router-dom";

import * as serviceWorker from './serviceWorker';

ReactDOM.render(
  <BrowserRouter>
    <Switch>
      <Route exact path="/" render={props => <App {...props} />} />
      <Route exact path="/conferences/:confID?" render={props => <Conferences {...props} />} />
      <Route exact path="/reviews" render={props => <Reviews {...props} />} />
    </Switch>
  </BrowserRouter>,
  document.getElementById('root')
);

// If you want your app to work offline and load faster, you can change
// unregister() to register() below. Note this comes with some pitfalls.
// Learn more about service workers: https://bit.ly/CRA-PWA
serviceWorker.unregister();
