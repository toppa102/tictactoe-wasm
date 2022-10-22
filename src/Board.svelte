<script lang="ts">
  import { createEventDispatcher } from "svelte";

  const dispatch = createEventDispatcher();

  export let board_size;
  export let win_amount;
  export let search_depth;

  import init, { check_winner, find_best_move } from "tictactoe-rs";
  import x from "./assets/x.svg";
  import o from "./assets/circle.svg";

  let wasm = init();

  $: board = new Uint8Array(new Array(board_size * board_size).fill(0));

  function playTurn(i: number) {
    board[i] = 2;
    board[find_best_move(board, board_size, win_amount, search_depth)] = 1;
    console.log(check_winner(board, board_size, win_amount));

    let winner = check_winner(board, board_size, win_amount);

    if (winner == 9223372036854775807n || winner == -9223372036854775808n) {
      board = new Uint8Array(new Array(board_size * board_size));
    }
  }
</script>

<main>
  {#await wasm}
    <div style="text-align: center; font-size: 3em;">Loading...</div>
  {:then}
    <div id="board" style="width: {board_size * 60 + board_size * 2}px;">
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
