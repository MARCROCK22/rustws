import test from 'ava'

import { createConnection } from '../index.js'

test('createConnection', (t) => {
  const c =
    createConnection({
      onMessage: (...message) => {
        console.log("message", ...message); // buildea. q deberia dar?
      },
      onClose: () => {
        console.log("close");
      },
      onError: (error) => {
        console.log("error", error);
      },
      onOpen: (...args) => {
        console.log("open", ...args);
      },
      url: 'wss://echo-websocket.hoppscotch.io'
    });
  c.print()
  t.is(true, true);
});


//rawCreateConnection();