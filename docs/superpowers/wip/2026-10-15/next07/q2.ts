// 問題2の検証: await 無しの PostgREST ビルダー風オブジェクトから data を取り出すとどうなるか
class FakeBuilder {
  then(resolve: (v: { data: string[]; error: null }) => void) {
    console.log("(通信開始)");
    resolve({ data: ["本"], error: null });
  }
}
const builder = new FakeBuilder();
const { data } = builder as unknown as { data?: string[] };
console.log("data =", data);
