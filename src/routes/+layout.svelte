<script lang="ts">
  import "../app.css";
  import { onMount } from "svelte";
  import { themeStore } from "$lib/theme/tokens.svelte";
  import Sidebar from "$lib/ui/Sidebar.svelte";

  let { children } = $props();

  onMount(() => {
    themeStore.mount();
    return () => themeStore.unmount();
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
