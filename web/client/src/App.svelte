<script lang="ts">
	import "bulma/css/bulma.css";
	import Router from "svelte-spa-router";
	import Navbar from "./layout/Navbar.svelte";

	import routes from "./routing";
</script>

<Navbar />

<Router {routes} />
