<script>
  import { Router, Route } from "svelte-routing"

  import Albums from "./lib/Albums.svelte"
  import Tracks from "./lib/Tracks.svelte";
</script>

<Router>
  <Route path="/albums" component={Albums} />
  <Route path="/albums/:id" component={Tracks} />
</Router>
