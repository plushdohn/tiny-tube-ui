<script lang="ts">
  import { createDialog, melt } from "@melt-ui/svelte";
  import { open as openBrowseDialog } from "@tauri-apps/api/dialog";
  import { FolderIcon } from "lucide-svelte";
  import { fade, scale } from "svelte/transition";

  export let dialog: ReturnType<typeof createDialog>;
  export let folderPath: string | null;

  let {
    elements: { overlay, content, portalled },
    states: { open },
  } = dialog;

  async function handleOpenFolderPath() {
    const choice = await openBrowseDialog({
      directory: true,
      multiple: false,
    });

    if (choice) {
      folderPath = choice as string;

      localStorage.setItem("download_path", folderPath);
    }
  }
</script>

<div use:portalled>
  {#if $open}
    <div
      use:melt={$overlay}
      class="fixed left-0 top-0 w-screen h-screen bg-zinc-950/75 backdrop-blur"
      transition:fade
    />
    <div
      class="fixed left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 flex flex-col w-full max-w-sm text-sm gap-4"
      transition:scale={{ duration: 500, start: 1.05 }}
      use:melt={$content}
    >
      <h1 class="text-xl">Settings</h1>
      <div class="flex flex-col gap-1 w-full">
        <label for="folder-path" class="text-zinc-400">Download folder</label>
        <div class="flex w-full border border-zinc-600 rounded overflow-hidden">
          <div class="bg-zinc-800 grow py-2 px-3" id="folder-path">
            {#if folderPath}
              <span class="text-zinc-400">{folderPath}</span>
            {:else}
              <span class="text-zinc-500">
                Choose a folder (default is Music)
              </span>
            {/if}
          </div>
          <button
            class="px-3 hover:bg-zinc-900 transition-colors focus:ring-1 ring-zinc-500"
            on:click={handleOpenFolderPath}
          >
            <FolderIcon class="w-4 h-4 text-zinc-400" />
          </button>
        </div>
      </div>
    </div>
  {/if}
</div>
