// http://127.0.0.1:54321/rest/v1/... -> PostgREST http://127.0.0.1:3000/...
// (Supabase's API gateway path prefix). Logs each request line to proxy.log.
import http from 'node:http';
import fs from 'node:fs';

http
  .createServer((req, res) => {
    const path = req.url.replace(/^\/rest\/v1/, '') || '/';
    fs.appendFileSync('proxy.log', `${req.method} ${req.url}\n`);
    const proxied = http.request(
      { host: '127.0.0.1', port: 3000, path, method: req.method, headers: req.headers },
      (up) => {
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
