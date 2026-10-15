// usage: node submit.mjs URL title   (JavaScript を使わない普通のフォーム送信を再現する)
const [url, title] = process.argv.slice(2);
const html = await (await fetch(url)).text();
const form = html.match(/<form[\s\S]*?<\/form>/)[0];
const fd = new FormData();
for (const m of form.matchAll(/<input type="hidden" name="([^"]+)"(?: value="([^"]*)")?\/>/g)) {
  fd.append(m[1], m[2] ?? "");
}
fd.append("title", title);
const res = await fetch(url, { method: "POST", body: fd, redirect: "manual" });
console.log("POST status:", res.status, res.headers.get("location") ?? "");
