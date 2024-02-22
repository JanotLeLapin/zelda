<script>
  import { Router, Route, Link } from "svelte-routing"

  import { audio } from "./lib/store"

  import Albums from "./lib/Albums.svelte"
  import Tracks from "./lib/Tracks.svelte"

  let paused = false;
</script>

<Router>
  <nav class="sidebar">
    <Link to="/albums">Albums</Link>
  </nav>

  <main>
    <Route path="/albums" component={Albums} />
    <Route path="/albums/:id" component={Tracks} />
  </main>
</Router>

<footer class="container">
  <div class="player">
    <audio
      src={$audio}
      bind:paused={paused}
    />
    <button on:click={() => paused = !paused}>Pause</button>
    </div>
</footer>

<style>
main {
  margin-left: 64px;
}

.sidebar {
  position: fixed;
  width: 64px;
}

footer {
  position: fixed;
  bottom: 0;
  width: 100vw;
  background-color: var(--bg-2);
  padding: 1rem;
}

.player {
  display: flex;
  justify-content: center;
}
</style>
