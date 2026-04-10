export default function StatCard(props: { label: string; value: number; accent: string }) {
  return (
    <div class="card overflow-hidden">
      <div class={`h-2 ${props.accent}`} />
      <div class="p-6">
        <p class="text-sm text-slate-500">{props.label}</p>
        <p class="mt-3 text-4xl font-semibold text-ink">{props.value}</p>
      </div>
    </div>
  );
}
