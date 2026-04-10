import type { JSX } from "solid-js";

export default function PageSection(props: {
  title: string;
  description?: string;
  actions?: JSX.Element;
  children: JSX.Element;
}) {
  return (
    <section class="space-y-4">
      <div class="flex flex-col gap-3 md:flex-row md:items-end md:justify-between">
        <div>
          <h2 class="text-2xl font-semibold">{props.title}</h2>
          {props.description ? <p class="mt-1 text-sm text-[var(--muted)]">{props.description}</p> : null}
        </div>
        {props.actions}
      </div>
      {props.children}
    </section>
  );
}
