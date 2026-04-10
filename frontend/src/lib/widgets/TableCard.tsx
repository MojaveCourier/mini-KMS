import type { JSX } from "solid-js";

export default function TableCard(props: { children: JSX.Element }) {
  return (
    <div class="overflow-hidden rounded-[24px] border border-[var(--line)] bg-white/80 shadow-[0_10px_35px_rgba(120,90,50,0.06)]">
      {props.children}
    </div>
  );
}
