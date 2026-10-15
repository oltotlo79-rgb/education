import { act } from "react";
export async function click(el: Element) {
  await act(async () => {
    el.dispatchEvent(new (window as any).MouseEvent("click", { bubbles: true }));
  });
}
export async function typeInto(el: HTMLInputElement, value: string) {
  const setter = Object.getOwnPropertyDescriptor((window as any).HTMLInputElement.prototype, "value")!.set!;
  await act(async () => {
    setter.call(el, value);
    el.dispatchEvent(new (window as any).Event("input", { bubbles: true }));
  });
}
export function text(el: Element) {
  return (el as HTMLElement).textContent;
}
