<script lang="ts" generics="T extends Format[]">
  import { fade } from "svelte/transition";
  import clsx from "clsx";
  import type { Format } from "../types";

  export let options: T;
  export let value: T[number];
  export let className: string = "";
</script>

<div class={clsx("flex gap-1 text-zinc-500", className)} in:fade>
  {#each options as opt, i}
    <button
      type="button"
      class={clsx(
        "text-xs transition-colors outline-none ring-zinc-600 focus:ring-1 rounded px-1",
        opt === value ? "text-zinc-100" : "hover:text-zinc-400"
      )}
      on:click={() => (value = opt)}
    >
      {opt}
    </button>
    {#if i < options.length - 1}
      <span>|</span>
    {/if}
  {/each}
</div>
