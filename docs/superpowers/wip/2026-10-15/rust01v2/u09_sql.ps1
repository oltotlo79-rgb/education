$body = '{"statements":["SELECT id, email FROM users","SELECT version FROM schema_migrations ORDER BY version"]}'
Invoke-RestMethod -Method Post -Uri http://127.0.0.1:8080/ -Body $body -ContentType application/json | ConvertTo-Json -Depth 6
