<script lang="ts">
    import Board from "./Board.svelte";
    let board_size = 3;
    let win_amount = 3;

    $: search_depth = Math.min(Math.max(10 - board_size, 2), 10);
</script>

<main>
    <div id="wrapper">
        Size: <input type="number" bind:value={board_size} />
        Amount for win: <input type="number" bind:value={win_amount} />
        <Board {board_size} {win_amount} {search_depth} />
    </div>
</main>

<style>
    #wrapper {
        margin: auto;
        width: 50%;
        text-align: center;
    }
    :global(body) {
        font-family: sans-serif;
        font-size: 1.15em;
    }
    input {
        font-size: inherit;
        width: 40px;
        margin-right: 10px;
    }
</style>
