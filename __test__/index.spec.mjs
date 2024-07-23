import test from 'ava'

import { createConnection, rawCreateConnection } from '../index.js'

test('createConnection', (t) => {
  const c =
    createConnection('rust');
  c.print()
  t.is(true, true);
});


rawCreateConnection();