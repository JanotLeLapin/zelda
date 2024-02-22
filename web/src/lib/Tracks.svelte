<script>
  import Play from "svelte-material-icons/Play.svelte"

  import { audio } from "./store"

  export let id;

  const root = "http://192.168.1.91:4000"
  const tracks = fetch(root + "/albums/" + encodeURIComponent(id)).then(res => res.json());
</script>

<main>
  <header>
    <img src={root + "/cover/" + encodeURIComponent(id)} alt="">
    <div>
      <p>Album</p>
    </div>
  </header>
  <div class="tracks">
    {#await tracks}
      Loading...
    {:then data}
      {#each data as track}
        <div class="track">
          <button on:click={() => audio.set(track)}><Play size={24} color="#fff" /></button>
          <h3>{track.name}</h3>
        </div>
      {/each}
    {/await}
  </div>
</main>

<style>
  header {
    display: flex;
    align-items: center;
    padding: 4rem;
  }

  header img {
    width: 256px;
    margin-right: 2rem;
  }

  .tracks {
    margin: 2rem;
  }

  .track {
    display: flex;
    margin: 1rem 0;
  }
</style>
