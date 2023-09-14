<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { createEventDispatcher, onMount } from "svelte";
  import type { Format } from "../types";
  import {
    CheckIcon,
    Loader2Icon,
    RotateCwIcon,
    XCircleIcon,
    XIcon,
  } from "lucide-svelte";

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
      <Loader2Icon class="animate-spin w-4 h-4 text-zinc-400" />
      <span class="text-zinc-400">Downloading metadata...</span>
    {:then metadata}
      {#await downloadPromise}
        <Loader2Icon class="animate-spin w-4 h-4 text-zinc-400" />
        <span class="text-zinc-400">{metadata.title}</span>
      {:then}
        <CheckIcon class="w-4 h-4 text-green-600" />
        <span class="text-zinc-400">{metadata.title}</span>
      {:catch}
        <XCircleIcon class="w-4 h-4 text-red-600" />
        <span class="text-zinc-400">{metadata.title}</span>
        <button on:click={downloadMetadata}>
          <RotateCwIcon class="w-4 h-4 text-zinc-400" />
        </button>
      {/await}
    {:catch}
      <XCircleIcon class="w-4 h-4 text-red-600" />
      <span class="text-zinc-400 w-full">Failed to download metadata.</span>
      <button
        on:click={downloadMetadata}
        class="outline-none ring-zinc-600 focus:ring-1 rounded p-0.5"
      >
        <RotateCwIcon class="w-4 h-4 text-zinc-400" />
      </button>
      <button
        on:click={() => dispatch("remove")}
        class="outline-none ring-zinc-600 focus:ring-1 rounded p-0.5"
      >
        <XIcon class="w-4 h-4 text-zinc-400" />
      </button>
    {/await}
  {/if}
</div>
