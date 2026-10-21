<script lang="ts">
  import "../app.css";
  import { onMount } from "svelte";
  import { themeStore } from "$lib/theme/tokens.svelte";
  import { progressStore, settingsStore } from "$lib/stores";
  import Sidebar from "$lib/ui/Sidebar.svelte";

  let { children } = $props();

  onMount(() => {
    // Applies the fallback default theme immediately; settingsStore.mount()
    // then swaps in the persisted theme once the async load resolves.
    themeStore.mount();
    void settingsStore.mount();
    // Mounted here, not per page: loading it is what evaluates and records
    // points, on startup and after every change, wherever the user is.
    void progressStore.mount();
    return () => {
      themeStore.unmount();
      settingsStore.unmount();
      progressStore.unmount();
    };
  });
</script>

<div class="shell">
  <Sidebar />
  <main class="content">
    {@render children()}
  </main>
</div>

<style>
  .shell {
    display: grid;
    grid-template-columns: 15rem 1fr;
    height: 100vh;
    overflow: hidden;
  }

  .content {
    display: flex;
    flex-direction: column;
    overflow-y: auto;
    background: var(--surface-base);
    position: relative;
  }
</style>
