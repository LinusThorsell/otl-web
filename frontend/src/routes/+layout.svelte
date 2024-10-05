<script>
  import "../app.css";
  import NavBar from "$lib/NavBar.svelte"
  import Footer from "$lib/Footer.svelte"
</script>

<div class="flex flex-col min-h-screen">
  <NavBar />

  <div class="grow dark:bg-gray-900">
    <slot />
  </div>

  <Footer />
</div>
