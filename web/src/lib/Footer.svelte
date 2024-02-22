<script>
  import Play from "svelte-material-icons/Play.svelte"
  import Pause from "svelte-material-icons/Pause.svelte"

  import SkipNext from "svelte-material-icons/SkipNext.svelte"

  import { queue } from "./store"

  const root = "http://192.168.1.91:4000"
  const size = 28;

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
      <div>
        <button class="primary" on:click={() => paused = !paused}>
          {#if paused}
            <Play {size} />
          {:else}
            <Pause {size} />
          {/if}
        </button>
        <button class="secondary" on:click={_ => queue.update(q => q.slice(1))}><SkipNext {size} /></button>
      </div>
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
}

footer button.primary {
  background-color: var(--fg-1);
  border-radius: 999px;
}

footer button.secondary {
  color: var(--fg-1);
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
