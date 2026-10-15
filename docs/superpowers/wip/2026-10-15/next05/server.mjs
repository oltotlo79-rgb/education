// Local "mini Supabase" for verifying chapter 5 samples:
// PGlite (real Postgres 17 in WASM) + pglite-socket, plus an HTTP proxy that maps
// /rest/v1/* -> PostgREST (started separately on port 3000).
import { PGlite } from '@electric-sql/pglite';
import { PGLiteSocketServer } from '@electric-sql/pglite-socket';
import http from 'node:http';

const db = await PGlite.create();

// Supabase-like roles / auth helpers (new 2026 default: NO automatic grants on public tables)
await db.exec(`
create role anon nologin noinherit;
create role authenticated nologin noinherit;
create role service_role nologin noinherit bypassrls;
create role authenticator noinherit login password 'postgres';
grant anon, authenticated, service_role to authenticator;
grant usage on schema public to anon, authenticated, service_role;
create schema auth;
grant usage on schema auth to anon, authenticated, service_role;
create table auth.users (id uuid primary key, email text);
create or replace function auth.uid() returns uuid language sql stable as $$
  select nullif(
    coalesce(
      current_setting('request.jwt.claim.sub', true),
      (current_setting('request.jwt.claims', true)::jsonb ->> 'sub')
    ), ''
  )::uuid
$$;
grant execute on function auth.uid() to anon, authenticated, service_role;
create function public.__ping() returns int language sql as 'select 1';
grant execute on function public.__ping() to anon, authenticated, service_role;
`);

const server = new PGLiteSocketServer({ db, port: 54329, host: '127.0.0.1', maxConnections: 8 });
await server.start();
console.log('PGlite listening on 127.0.0.1:54329');

// Proxy: http://127.0.0.1:54321/rest/v1/... -> http://127.0.0.1:3000/...
// Workaround for a pglite-socket quirk: after PostgREST returns an error, the next DB use
// fails once with PGRST001. The proxy sends one throwaway request after any error response
// so that the reader-facing requests always get the genuine PostgREST answer.
let healing = Promise.resolve();
function ping() {
  return new Promise((resolve) => {
    const r = http.request({ host: '127.0.0.1', port: 3000, path: '/rpc/__ping', method: 'GET' }, (up) => {
      up.resume();
      up.on('end', () => resolve(up.statusCode));
    });
    r.on('error', () => resolve(0));
    r.end();
  });
}
async function throwaway() {
  for (let i = 0; i < 5; i++) {
    const st = await ping();
    if (st === 200) return;
    await new Promise((r) => setTimeout(r, 200));
  }
}
http
  .createServer(async (req, res) => {
    await healing;
    const path = req.url.replace(/^\/rest\/v1/, '') || '/';
    const proxied = http.request(
      { host: '127.0.0.1', port: 3000, path, method: req.method, headers: req.headers },
      (up) => {
        if (up.statusCode >= 400) healing = throwaway();
        res.writeHead(up.statusCode, up.headers);
        up.pipe(res);
      }
    );
    proxied.on('error', (e) => {
      res.writeHead(502);
      res.end(String(e));
    });
    req.pipe(proxied);
  })
  .listen(54321, '127.0.0.1', () => console.log('proxy on 127.0.0.1:54321'));

process.on('uncaughtException', (e) => console.error('uncaught', e.code || e));
