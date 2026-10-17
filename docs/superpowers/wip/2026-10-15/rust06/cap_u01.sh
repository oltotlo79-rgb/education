source "$(dirname "$0")/tools/lib.sh"
cd "$LAB"
cargo build --bin u01_raw --bin u01_status 2>&1 | tail -1
# 最小: u01_raw
start_bin u01_raw $P
{ c -i "$B/hello?name=ada"; } > "$R6/out/u01_raw_curl.txt"
{ "$CURL" -s -v "$B/hello?name=ada" -o NUL 2>&1 | p3k; } > "$R6/out/u01_raw_curl_v.txt"
{ c -i -d "title=買い物&body=牛乳" "$B/notes"; } > /dev/null
stop_bin u01_raw
grep -v "^$" "$R6/out/_server_u01_raw.log" | p3k > "$R6/out/u01_raw_server.txt"
# もう一歩: u01_status
start_bin u01_status $P
{
for a in "/" "/nothing" "/admin" "/old-page"; do echo "### curl.exe -i http://127.0.0.1:3000$a"; c -i "$B$a"; done
echo "### curl.exe -i -X DELETE http://127.0.0.1:3000/"; c -i -X DELETE "$B/"
echo "### curl.exe -i -d message=hello http://127.0.0.1:3000/messages"; c -i -d message=hello "$B/messages"
echo "### curl.exe -i -L -d message=hello http://127.0.0.1:3000/messages"; c -i -L -d message=hello "$B/messages"
echo "### curl.exe -i -L -X POST -d message=hello http://127.0.0.1:3000/messages"; c -i -L -X POST -d message=hello "$B/messages"
echo "### curl.exe -I http://127.0.0.1:3000/"; c -I "$B/"
} > "$R6/out/u01_status_curl.txt"
stop_bin u01_status
cat "$R6/out/_server_u01_status.log" | p3k > "$R6/out/u01_status_server.txt"
# len bug
cp "$R6/errs/u01_len_bug.rs" src/bin/u01_len_bug.rs
cargo build --bin u01_len_bug 2>&1 | tail -1
start_bin u01_len_bug $P
{ c -i "$B/"; } > "$R6/out/u01_len_bug_curl.txt"
stop_bin u01_len_bug
rm src/bin/u01_len_bug.rs
# 接続拒否（何も待っていないポート）
{ "$CURL" -i http://127.0.0.1:$P/ 2>&1 | p3k; } > "$R6/out/u01_refused.txt"
