<script lang="ts">
    const plugins = [
        {
            name: "Test Plugin 1",
            desc: "This is Test Plugin 1",
        },
        {
            name: "Test Plugin 2",
            desc: "This is Test Plugin 2",
        },
        {
            name: "Test Plugin 3",
            desc: "This is Test Plugin 3",
        },
        {
            name: "Test Plugin 4",
            desc: "This is Test Plugin 4",
        },
        {
            name: "Test Plugin 5",
            desc: "This is Test Plugin 5",
        },
        {
            name: "Test Plugin 6",
            desc: "This is Test Plugin 6",
        },
        {
            name: "Test Plugin 7",
            desc: "This is Test Plugin 7",
        },
        {
            name: "Test Plugin 8",
            desc: "This is Test Plugin 8",
        },
        {
            name: "Test Plugin 9",
            desc: "This is Test Plugin 9",
        },
        {
            name: "Test Plugin 10",
            desc: "This is Test Plugin 10",
        },
        {
            name: "Test Plugin 11",
            desc: "This is Test Plugin 11",
        },
        {
            name: "Test Plugin 12",
            desc: "This is Test Plugin 12",
        },
    ];
</script>

<section class="section">
    <div class="container">
        <h1 class="title">Plugins</h1>
        <h3 class="subtitle">Here are the plugins!</h3>
    </div>
</section>

<hr class="divider" />

<section class="section">
    <div class="container">
        <div class="columns is-multiline">
            {#each plugins as plugin}
                <div class="column is-one-third">
                    <div class="box">
                        <p class="title is-spaced">{plugin.name}</p>
                        <p class="subtitle">{plugin.desc}</p>
                    </div>
                </div>
            {/each}
        </div>
    </div>
</section>
