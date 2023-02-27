<script lang="ts">
  export let results: InputResult[];
  export let onExtension: ((Component: ComponentType) => void) | undefined = undefined;

  import { appWindow, LogicalSize } from '@tauri-apps/api/window';
  import { convertFileSrc, invoke } from '@tauri-apps/api/tauri';
  import { afterUpdate, type ComponentType } from 'svelte';
  import CalculationResult from './CalculationResult.svelte';
  import FileSearchResult from './FileSearchResult.svelte';
  import { ResultType, type InputResult } from '../../../utils/result';

  let Component: ComponentType | null = null;

  afterUpdate(async () => {
    const height = document.getElementsByClassName('container')[0].clientHeight;
    await appWindow.setSize(new LogicalSize(750, height));
    if (results.length > 0 && results[0].value !== '') {
      const firstResult = document.getElementById(results[0].value);
      if (firstResult) {
        firstResult.classList.add('searchResultFocused');
        await firstResult.focus();
      }
    }
  });

  const searchResultClicked = async (event: any) => {
    const result = results.find((it) => it.value === event.target.id);

    if (result.type === ResultType.Extensions) {
      try {
        const path = convertFileSrc(result.value);
        if (onExtension) {
          onExtension((await import(path)).Component);
        }
      } catch (e) {
        console.error(e);
      }
      return;
    }

    await invoke('open_command', { path: event.target.id });
    const searchBarInput = document.getElementById(
      'searchBarInput'
    ) as HTMLInputElement;
    results = [];
    searchBarInput.value = '';
    await appWindow.hide();
  };

  async function handleKeydown(event) {
    if (event.keyCode == 38 || event.keyCode == 40) {
      const current = document.activeElement as HTMLElement | null;
      const items = [...document.getElementsByClassName('searchResult')] as
        | HTMLElement[]
        | null;
      const currentIndex = items.indexOf(current);
      let newIndex;

      if (currentIndex === -1) {
        newIndex = 0;
      } else {
        if (event.keyCode === 38)
          newIndex = (currentIndex + items.length - 1) % items.length;
        else newIndex = (currentIndex + 1) % items.length;
      }
      if (current !== null && items[newIndex] !== null) {
        items[newIndex].classList.add('searchResultFocused');
        current.classList.remove('searchResultFocused');
        current.blur();
        items[newIndex].focus();
      }
    } else if (event.key == 'Enter') {
      const current = document.activeElement as HTMLElement | null;
      if (current !== null) {
        current.click();
      }
    } else {
      const searchBarInput = document.getElementById(
        'searchBarInput'
      ) as HTMLInputElement | null;
      searchBarInput.focus();
    }
  }
</script>

<svelte:window on:keydown={handleKeydown} />

<div class="searchResults">
  {#if results.length > 0 && results[0].value !== ' '}
    {#if results[0].type !== ResultType.Calculation}
      {#each results.slice(0, 5) as result}
        <FileSearchResult
          filePath={result}
          on:click={searchResultClicked}
        />
      {/each}
    {:else}
      <CalculationResult bind:results />
    {/if}
  {/if}
</div>

<style>
  :global(.searchResultFocused) {
    background: var(--highlight-overlay) !important;
    outline: 0 !important;
    border-radius: 8px;
  }
</style>
