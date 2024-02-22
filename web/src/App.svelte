<script>
  const root = "http://192.168.1.91:4000"
  const albums = fetch(root).then(res => res.json());
</script>

<main>
  <div class="albums">
    {#await albums}
      <p>Loading...</p>
    {:then data}
      {#each data as album}
        <div class="album">
          <img src={root + "/cover/" + encodeURIComponent(album.path)} alt="">
          <h3>{album.name}</h3>
        </div>
      {/each}
    {:catch}
      <p>Something went wrong...</p>
    {/await}
  </div>
</main>

<style>
.album {
  display: flex;
  align-items: center;
  gap: 1rem;
  margin: 1rem;
}

.album img {
  width: 48px;
  border-radius: 4px;
}
</style>
