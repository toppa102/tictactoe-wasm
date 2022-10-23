<script lang="ts">
    import Board from "./Board.svelte";
    import { options } from "./store";

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
        Size: <input
            type="number"
            bind:value={$options.size}
            on:change={() =>
                ($options.win = Math.min(
                    Math.max($options.win, 1),
                    $options.size
                ))}
            min="1"
        />
        Amount for win:
        <input
            type="number"
            bind:value={$options.win}
            min="1"
            max={$options.size}
        />
        <Board options={$options} on:message={handleMessage} />
    </div>
</main>

<style>
    #wrapper {
        margin: auto;
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
