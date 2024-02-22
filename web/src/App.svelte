<script>
  import { Router, Route } from "svelte-routing"

  import { audio } from "./lib/store"

  import Albums from "./lib/Albums.svelte"
  import Tracks from "./lib/Tracks.svelte"
</script>

<Router>
  <Route path="/albums" component={Albums} />
  <Route path="/albums/:id" component={Tracks} />
</Router>

<footer class="player">
  <audio autoplay controls src={$audio}></audio>
</footer>

<style>
footer {
  position: fixed;
  bottom: 0;
  width: 100vw;
  background-color: var(--bg-2);
  padding: 1rem;
}
</style>
