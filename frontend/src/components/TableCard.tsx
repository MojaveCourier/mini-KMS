import { type JSX } from "solid-js";

export default function TableCard(props: { title: string; action?: JSX.Element; children: JSX.Element }) {
  return (
    <section class="card p-6">
      <div class="mb-5 flex items-center justify-between gap-3">
        <h2 class="text-xl font-semibold text-ink">{props.title}</h2>
        {props.action}
      </div>
      <div class="overflow-x-auto">{props.children}</div>
    </section>
  );
}
