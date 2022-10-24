<script lang="ts">
  import { createEventDispatcher } from "svelte";

  const dispatch = createEventDispatcher();

  export let options;

  import init, { check_winner, find_best_move } from "tictactoe-rs";
  import x from "./assets/x.svg";
  import o from "./assets/circle.svg";

  let wasm = init();

  $: board = new Uint8Array(new Array(options.size * options.size).fill(0));
  $: depth = 11 - options.size;

  function checkWinner() {
    let winner = check_winner(board, options.size, options.win);

    if (winner == 9223372036854775807n) {
      dispatch("message", { winner: "ai" });
      board = new Uint8Array(new Array(options.size * options.size));
      return 1;
    }
    if (winner == -9223372036854775808n) {
      dispatch("message", { winner: "player" });
      board = new Uint8Array(new Array(options.size * options.size));
      return 2;
    }
    if (board.every((x) => x != 0)) {
      dispatch("message", { winner: "draw" });
      board = new Uint8Array(new Array(options.size * options.size));
      return 3;
    }
    return 0;
  }

  function playTurn(i: number) {
    board[i] = 2;
    if (checkWinner() != 0) return;
    board[find_best_move(board, options.size, options.win, depth)] = 1;
    checkWinner();
  }
</script>

<main>
  {#await wasm}
    <div style="text-align: center; font-size: 3em;">Loading...</div>
  {:then}
    <div id="board" style="width: {options.size * 60 + options.size * 2}px;">
      {#each board as val, i}
        {#if val == 1}
          <div class="cell"><img src={o} alt="O" /></div>
        {/if}
        {#if val == 2}
          <div class="cell"><img src={x} alt="X" /></div>
        {/if}
        {#if val == 0}
          <div class="cell">
            <input
              type="button"
              on:click={() => {
                playTurn(i);
              }}
            />
          </div>
        {/if}
      {/each}
    </div>
  {/await}
</main>

<style>
  main {
    padding: 10px;
  }
  #board {
    margin: auto;
    display: flex;
    flex-wrap: wrap;
  }
  .cell {
    flex: 1;
    min-width: 60px;
    min-height: 60px;
    border: 1px solid gray;
  }

  img {
    display: block;
    margin: auto;
    width: 100%;
    height: 100%;
  }

  input {
    background-color: transparent;
    width: 100%;
    height: 100%;
    border: 0;
  }
</style>
