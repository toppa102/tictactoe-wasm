<script lang="ts">
    import Board from "./Board.svelte";
    let board_size = 3;
    let win_amount = 3;

    $: search_depth = Math.min(Math.max(11 - board_size, 2), 11);

    function handleMessage(event) {
        if (event.detail.winner == "player") {
            alert("You won!");
        } else if (event.detail.winner == "ai") {
            alert("You lost!");
        } else {
            alert("Draw!");
        }
    }
</script>

<main>
    <div id="wrapper">
        Size: <input type="number" bind:value={board_size} min="1" />
        Amount for win:
        <input type="number" bind:value={win_amount} min="1" max={board_size} />
        <Board
            {board_size}
            {win_amount}
            {search_depth}
            on:message={handleMessage}
        />
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
