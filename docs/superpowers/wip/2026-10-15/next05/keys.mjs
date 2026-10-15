// HS256 JWT keys equivalent to Supabase legacy anon / service_role keys (local only)
import crypto from 'node:crypto';

const SECRET = 'super-secret-jwt-token-with-at-least-32-characters-long';
const b64 = (o) => Buffer.from(JSON.stringify(o)).toString('base64url');

export function sign(payload) {
  const head = b64({ alg: 'HS256', typ: 'JWT' });
  const body = b64({ iss: 'supabase-local', exp: 4102444800, ...payload });
  const sig = crypto.createHmac('sha256', SECRET).update(`${head}.${body}`).digest('base64url');
  return `${head}.${body}.${sig}`;
}

export const ANON_KEY = sign({ role: 'anon' });
export const SERVICE_ROLE_KEY = sign({ role: 'service_role' });
export const URL = 'http://127.0.0.1:54321';
