<script lang="ts">
  import Board from "./Board.svelte";
  import { options } from "./options-store";
</script>

<main>
  <div id="wrapper">
    Size:
    <input
      type="number"
      bind:value={$options.size}
      on:change={() =>
        ($options.win = Math.min(Math.max($options.win, 1), $options.size))}
      min="1"
      style="width: 40px;"
    />
    Amount for win:
    <input
      type="number"
      bind:value={$options.win}
      min="1"
      max={$options.size}
      style="width: 40px;"
    />

    <Board options={$options} />
  </div>
</main>

<style>
  #wrapper {
    margin: auto;
    text-align: center;
  }
  :global(body) {
    font-family: sans-serif;
    font-size: 1.3em;
  }
  input {
    font-size: inherit;
    margin-right: 10px;
  }
</style>
