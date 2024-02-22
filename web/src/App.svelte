<script>
  import { Router, Route, Link } from "svelte-routing"

  import Home from "svelte-material-icons/Home.svelte"
  import Album from "svelte-material-icons/Album.svelte"

  import Play from "svelte-material-icons/Play.svelte"
  import Pause from "svelte-material-icons/Pause.svelte"

  import { queue } from "./lib/store"

  import Albums from "./lib/Albums.svelte"
  import Tracks from "./lib/Tracks.svelte"

  const root = "http://192.168.1.91:4000"

  let paused = true;
  let duration = 1;

  let time = 0;

  function setTimeDom(seconds) {
		let element = document.getElementById('audio')
		element.currentTime = seconds
	}

  let track = null;
  queue.subscribe(v => track = v.at(0) || null);
</script>

<Router>
  <nav class="sidebar">
    <Link to="/"><Home size={32} /></Link>
    <Link to="/albums"><Album size={32} /></Link>
  </nav>

  <main>
    <Route path="/albums" component={Albums} />
    <Route path="/albums/:id" component={Tracks} />
  </main>
</Router>

<footer class="container">
  <div class="player">
    <div class="current">
      {#if track}
        <img src={root + "/cover/" + encodeURIComponent(track.album)} alt="">
        <h3>{track.name}</h3>
        <audio
          id="audio"
          src={track.path ? root + "/stream/" + encodeURIComponent(track.path) : null}
          bind:paused={paused}
          bind:duration={duration}
          on:timeupdate={e => time = e.currentTarget.currentTime}
          on:ended={_ => queue.update(q => q.slice(1))}
          autoplay
        />
      {/if}
    </div>
    <div class="controls">
      <button class="play" on:click={() => paused = !paused}>
        {#if paused}
          <Play size={28} />
        {:else}
          <Pause size={28} />
        {/if}
      </button>
      <br />
      <input
        type="range"
        min={0} max={duration}
        bind:value={time}
        on:input={e => setTimeDom(parseInt(e.currentTarget.value))}
      />
    </div>
    <div class="extra"></div>
  </div>
</footer>

<style>
.sidebar {
  position: fixed;
  top: 0;
  left: 0;
  padding: 16px;
  width: 64px;
}

main {
  margin: 2rem 0 12rem 128px;
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
  justify-content: space-between;
  align-items: center;
}

.current {
  display: flex;
  align-items: center;
}

.current img {
  border-radius: 4px;
  width: 64px;
  margin-right: 1rem;
}

.controls {
  text-align: center;
}
</style>
