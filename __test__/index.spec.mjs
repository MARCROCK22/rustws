import test from 'ava'

import { createConnection } from '../index.js'

test('createConnection', (t) => {
  const c =
    createConnection({
      onMessage: (...message) => {
        console.log("message", ...message); // buildea. q deberia dar?
        c.send('xd');
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

  console.log(c.send.toString())

  /*setTimeout(() => {
    c.send('xd')
    console.log('Message')
  }, 1000)*/

  t.is(true, true);
});


//rawCreateConnection();