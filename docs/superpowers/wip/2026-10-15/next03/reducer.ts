// 状態の形
type CounterState = { count: number };

// 操作（action）の種類
type CounterAction =
  | { type: "increment" }
  | { type: "decrement" }
  | { type: "reset" };

// 6.10 と同じ reducer 関数（React を使わない、ただの関数）
function counterReducer(state: CounterState, action: CounterAction): CounterState {
  switch (action.type) {
    case "increment":
      return { count: state.count + 1 };
    case "decrement":
      return { count: state.count - 1 };
    case "reset":
      return { count: 0 };
    default:
      return state;
  }
}

// ボタンを「+1, +1, -1, リセット」の順に押したつもりの action の列
const actions: CounterAction[] = [
  { type: "increment" },
  { type: "increment" },
  { type: "decrement" },
  { type: "reset" },
];

// dispatch の代わりに、action を1つずつ reducer に通す
let state: CounterState = { count: 0 };
for (const action of actions) {
  const next = counterReducer(state, action);
  console.log(`${action.type}: ${state.count} → ${next.count}`);
  state = next;
}

// 配列の reduce を使うと「最終状態」を一度に計算できる（名前の由来）
const finalState = actions.reduce(counterReducer, { count: 0 });
console.log("reduce で計算した最終状態:", finalState);
