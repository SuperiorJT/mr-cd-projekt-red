import Dashboard from "./routes/Dashboard.svelte";
import Settings from "./routes/Settings.svelte";
import Plugins from "./routes/Plugins.svelte";
import NotFound from "./routes/404.svelte";

export default {
    "/": Dashboard,
    "/settings": Settings,
    "/plugins": Plugins,

    "*": NotFound
}