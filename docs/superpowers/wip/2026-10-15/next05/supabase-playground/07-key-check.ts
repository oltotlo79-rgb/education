// 07-key-check.ts : .env.local のキーの置き方が安全かを点検する
// 実行方法: node --env-file=.env.local 07-key-check.ts
// （通信はしません。手元の環境変数を調べるだけです）

// キーの文字列を見て「公開用」か「秘密」かを判定する関数
function keyKind(value: string): string {
  if (value.startsWith('sb_publishable_')) {
    return '公開用（Publishable key）';
  }
  if (value.startsWith('sb_secret_')) {
    return '秘密（Secret key）';
  }
  // 旧形式のキーは「.」で3つに区切られた JWT で、真ん中の部分に role が書かれている
  if (value.startsWith('eyJ')) {
    try {
      const middle = value.split('.')[1];
      const payload = JSON.parse(Buffer.from(middle, 'base64url').toString('utf8'));
      if (payload.role === 'anon') {
        return '公開用（旧 anon key）';
      }
      if (payload.role === 'service_role') {
        return '秘密（旧 service_role key）';
      }
    } catch {
      return '不明（JWT として読めません）';
    }
  }
  return '不明';
}

// 見つかった問題の数
let problems = 0;

// すべての環境変数のうち、名前に SUPABASE を含み、_KEY で終わるものを調べる
for (const [name, value] of Object.entries(process.env)) {
  if (!value || !name.includes('SUPABASE') || !name.endsWith('_KEY')) {
    continue;
  }
  const kind = keyKind(value);
  // NEXT_PUBLIC_ 付きの変数は、ブラウザに送られる
  const goesToBrowser = name.startsWith('NEXT_PUBLIC_');
  // 秘密キーがブラウザに送られる設定になっていたら NG
  const ok = !(goesToBrowser && kind.startsWith('秘密'));
  if (!ok) {
    problems++;
  }
  const mark = ok ? 'OK' : 'NG';
  const advice = ok ? '' : '  ← 秘密キーに NEXT_PUBLIC_ を付けてはいけません';
  console.log(`${mark}  ${name} : ${kind}${advice}`);
}

if (problems === 0) {
  console.log('問題は見つかりませんでした');
} else {
  console.log(`${problems}件の問題があります。キーを作り直す（ローテーション）ことも検討してください`);
}
