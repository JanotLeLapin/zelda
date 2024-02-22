<script>
  import { Link } from "svelte-routing";

  const root = "http://192.168.1.91:4000"
  const albums = fetch(root + "/albums").then(res => res.json());
</script>

<main>
  <h1>Albums</h1>
  <div class="albums">
    {#await albums}
      <p>Loading...</p>
    {:then data}
      {#each data as album}
        <div class="album">
          <Link to={"/albums/" + encodeURIComponent(album.path)}>
            <img src={root + "/cover/" + encodeURIComponent(album.path)} alt="">
            <h3>{album.name}</h3>
          </Link>
        </div>
      {/each}
    {/await}
  </div>
</main>

<style>
.albums {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
  gap: 2rem;
  margin: 2rem;
}

.album {
  align-items: center;
  border-radius: 4px;
  background-color: var(--bg-2);
  gap: 1rem;
  padding: 1rem;
}

.album h3 {
  margin: 1rem 0;
}

.album img {
  width: 100%;
  border-radius: 4px;
}
</style>
