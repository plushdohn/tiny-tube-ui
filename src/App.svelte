<script lang="ts">
  import { onMount } from "svelte";
  import { GithubIcon, SettingsIcon } from "lucide-svelte";
  import { open } from "@tauri-apps/api/shell";
  import { createDialog, melt } from "@melt-ui/svelte";
  import type { Format } from "./types";
  import TagSelect from "./lib/TagSelect.svelte";
  import DownloadEntry from "./lib/DownloadEntry.svelte";
  import SettingsDialog from "./lib/SettingsDialog.svelte";

  let format: Format = "mp3";
  let url = "";
  let downloads: Array<{ url: string; format: Format }> = [];
  let folderPath: string | null = null;

  const settingsDialog = createDialog();

  const { trigger } = settingsDialog.elements;

  onMount(() => {
    folderPath = localStorage.getItem("download_path");
  });

  function handlePaste(e: ClipboardEvent) {
    const clipboardData = e.clipboardData;
    const pastedText = clipboardData?.getData("text");

    if (!pastedText) return;

    downloads = [
      ...downloads,
      {
        url: pastedText,
        format,
      },
    ];
  }

  function handleSubmit(e: Event) {
    e.preventDefault();

    if (!url) return;

    downloads = [
      ...downloads,
      {
        url,
        format,
      },
    ];

    url = "";
  }

  function removeDownload(url: string) {
    downloads = downloads.filter((download) => download.url !== url);
  }
</script>

<main class="min-h-[100dvh] bg-zinc-900 flex flex-col text-sm p-2">
  <div
    class="w-full max-w-sm flex flex-col gap-4 grow self-center justify-center"
  >
    <form on:submit={handleSubmit} class="flex flex-col gap-2">
      <TagSelect
        options={["mp3", "mp4"]}
        bind:value={format}
        className="self-end"
      />
      <input
        placeholder="Paste a video or playlist link..."
        bind:value={url}
        on:paste={handlePaste}
        class="py-2 px-3 rounded border bg-zinc-800 border-zinc-700 text-zinc-100 outline-none focus:ring-1 ring-zinc-600"
      />
    </form>
    {#if downloads.length > 0}
      <hr class="w-full border-zinc-700" />
    {/if}
    <div class="flex flex-col gap-2 max-h-96 overflow-y-auto pr-2">
      {#each downloads as download}
        <DownloadEntry
          {...download}
          on:remove={() => removeDownload(download.url)}
          {folderPath}
        />
      {/each}
    </div>
  </div>
  <footer class="text-xs flex justify-end items-center gap-1">
    <button
      on:click={() => open("https://github.com/plushdohn/tiny-tube-ui")}
      class="outline-none focus:ring-1 ring-zinc-600 rounded p-0.5"
    >
      <GithubIcon
        class="w-4 h-4 text-zinc-600 hover:text-zinc-400 transition-colors"
      />
    </button>
    <button
      class="outline-none focus:ring-1 ring-zinc-600 rounded p-0.5"
      use:melt={$trigger}
    >
      <SettingsIcon
        class="w-4 h-4 text-zinc-600 hover:text-zinc-400 transition-colors"
      />
    </button>
  </footer>
  <SettingsDialog dialog={settingsDialog} bind:folderPath />
</main>
