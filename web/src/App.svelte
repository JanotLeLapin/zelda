<script>
  import { Router, Route, Link } from "svelte-routing"
  import Play from "svelte-material-icons/Play.svelte"
  import Pause from "svelte-material-icons/Pause.svelte"
  import Album from "svelte-material-icons/Album.svelte"

  import { audio } from "./lib/store"

  import Albums from "./lib/Albums.svelte"
  import Tracks from "./lib/Tracks.svelte"

  let paused = true;
</script>

<Router>
  <nav class="sidebar">
    <Link to="/albums"><Album size={32} /></Link>
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
      autoplay
    />
    <button class="play" on:click={() => paused = !paused}>
      {#if paused}
        <Play size={28} />
      {:else}
        <Pause size={28} />
      {/if}
    </button>
  </div>
</footer>

<style>
main {
  margin-left: 64px;
}

.sidebar {
  position: fixed;
  padding: 16px;
  width: 64px;
}

footer {
  position: fixed;
  bottom: 0;
  width: 100vw;
  background-color: var(--bg-2);
  padding: 1rem;
}

footer button {
  width: 40px;
  height: 40px;
  background-color: var(--fg-1);
  border-radius: 999px;
}

.player {
  display: flex;
  justify-content: center;
}
</style>
