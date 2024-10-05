<script>
  import { page } from '$app/stores';
  import Container from '$lib/Container.svelte';
</script>

<Container>
  <h1 class="text-4xl dark:text-white">Attans! Något gick fel!</h1>
  {#if $page.status === 404}
    <h1 class="mt-4 text-2xl dark:text-white">Felkod: {$page.status}</h1>
    <h1 class="text-2xl dark:text-white">Sidan hittades inte</h1>
  {:else}
    <h1 class="mt-4 text-2xl dark:text-white">Felkod: {$page.status}</h1>
    <h1 class="text-2xl dark:text-white">{$page.error.message}</h1>
  {/if}
</Container>
