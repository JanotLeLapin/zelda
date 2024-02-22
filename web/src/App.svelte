<script>
  import { Router, Route, Link } from "svelte-routing"

  import Footer from "./lib/Footer.svelte"

  import Home from "svelte-material-icons/Home.svelte"
  import Album from "svelte-material-icons/Album.svelte"

  import Albums from "./lib/Albums.svelte"
  import Tracks from "./lib/Tracks.svelte"
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

  <Footer />
</Router>

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
</style>
