<script>
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
    margin: 1rem 0;
  }
</style>
