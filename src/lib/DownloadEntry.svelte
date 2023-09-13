<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import {
    IconCheck,
    IconCircleX,
    IconLoader2,
    IconRotate2,
    IconX,
  } from "@tabler/icons-svelte";
  import { createEventDispatcher, onMount } from "svelte";
  import type { Format } from "../types";

  export let folderPath: string | null = null;
  export let url: string;
  export let format: Format;

  let metadataPromise: Promise<{ title: string }>;
  let downloadPromise: Promise<void>;

  const dispatch = createEventDispatcher();

  onMount(() => {
    downloadMetadata();
  });

  function downloadMetadata() {
    metadataPromise = invoke("download_metadata", { url })
      .then((metadata) => {
        downloadVideo();

        return metadata as { title: string };
      })
      .catch((err) => {
        console.error(err);
        throw err;
      });
  }

  function downloadVideo() {
    downloadPromise = invoke("download_video", {
      url,
      audioOnly: format === "mp3",
      folderPath,
    });
  }
</script>

<div class="py-2 px-3 rounded flex gap-2 items-center border border-zinc-700">
  {#if metadataPromise}
    {#await metadataPromise}
      <IconLoader2 class="animate-spin w-4 h-4 text-zinc-400" />
      <span class="text-zinc-400">Downloading metadata...</span>
    {:then metadata}
      {#await downloadPromise}
        <IconLoader2 class="animate-spin w-4 h-4 text-zinc-400" />
        <span class="text-zinc-400">{metadata.title}</span>
      {:then}
        <IconCheck class="w-4 h-4 text-green-600" />
        <span class="text-zinc-400">{metadata.title}</span>
      {:catch}
        <IconCircleX class="w-4 h-4 text-red-600" />
        <span class="text-zinc-400">{metadata.title}</span>
        <button on:click={downloadMetadata}>
          <IconRotate2 class="w-4 h-4 text-zinc-400" />
        </button>
      {/await}
    {:catch}
      <IconCircleX class="w-4 h-4 text-red-600" />
      <span class="text-zinc-400 w-full">Failed to download metadata.</span>
      <button
        on:click={downloadMetadata}
        class="outline-none ring-zinc-600 focus:ring-1 rounded p-0.5"
      >
        <IconRotate2 class="w-4 h-4 text-zinc-400" />
      </button>
      <button
        on:click={() => dispatch("remove")}
        class="outline-none ring-zinc-600 focus:ring-1 rounded p-0.5"
      >
        <IconX class="w-4 h-4 text-zinc-400" />
      </button>
    {/await}
  {/if}
</div>
