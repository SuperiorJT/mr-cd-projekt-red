<script lang="ts">
    let menuActive = false;
    function toggleMenu(e) {
        menuActive = !menuActive;
    }
</script>

<!-- svelte-ignore a11y-missing-attribute -->
<nav class="navbar" role="navigation" aria-label="main navigation">
    <div class="navbar-brand">
        <a class="navbar-item" href="/#/">
            <h3 class="is-size-3 is-hidden-mobile">Mr. CD Projekt Red</h3>
            <h3 class="is-size-3 is-hidden-tablet">CDPR</h3>
        </a>

        <a
            class:is-active={menuActive}
            on:click={toggleMenu}
            role="button"
            class="navbar-burger burger"
            aria-label="menu"
            aria-expanded="false">
            <span aria-hidden="true" />
            <span aria-hidden="true" />
            <span aria-hidden="true" />
        </a>
    </div>

    <div class="navbar-menu" class:is-active={menuActive}>
        <div class="navbar-start">
            <a class="navbar-item" href="/#/" on:click={toggleMenu}> Dashboard </a>
            
            <a class="navbar-item" href="/#/plugins" on:click={toggleMenu}> Plugins </a>

            <a class="navbar-item" href="/#/settings" on:click={toggleMenu}> Settings </a>

            <!-- <div class="navbar-item has-dropdown is-hoverable">
                <a class="navbar-link"> More </a>

                <div class="navbar-dropdown">
                    <a class="navbar-item"> About </a>
                    <a class="navbar-item"> Jobs </a>
                    <a class="navbar-item"> Contact </a>
                    <hr class="navbar-divider" />
                    <a class="navbar-item"> Report an issue </a>
                </div>
            </div> -->
        </div>
    </div>
</nav>
